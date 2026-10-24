import json
import shlex
import datetime

driver_retry = retry

def retry(condition):
    driver_retry(condition, timeout=datetime.timedelta(seconds=60))


def user(machine, command):
    env = "XDG_RUNTIME_DIR=/run/user/1000 SWAYSOCK=/tmp/sway-ipc.sock DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus"
    return machine.succeed("su - alice -c " + shlex.quote(env + " " + command))


def ipc(machine, command="", kind="command"):
    result = json.loads(user(machine, "swaymsg -r -t " + kind + " -- " + shlex.quote(command)))
    if kind == "command":
        assert all(item["success"] for item in result), (command, result)
    return result


def walk(tree):
    yield tree
    for group in ("nodes", "floating_nodes"):
        for node in tree.get(group, []):
            yield from walk(node)


def window(machine, name):
    return next((node for node in walk(ipc(machine, kind="get_tree")) if node.get("name") == name), None)


def exercise(machine):
    machine.wait_for_unit("multi-user.target")
    machine.wait_for_file("/tmp/sway-ipc.sock", timeout=120)
    machine.wait_until_succeeds("pgrep -x swaybar")
    outputs = [output for output in ipc(machine, kind="get_outputs") if output["active"]]
    assert len(outputs) == 1 and outputs[0]["active"], outputs
    assert outputs[0]["name"].startswith("Virtual"), outputs
    inputs = ipc(machine, kind="get_inputs")
    assert any(device["type"] == "keyboard" for device in inputs), inputs
    assert ipc(machine, kind="get_bar_config") == ["vm"]

    # The QEMU keyboard, libinput, key bindings, focus, and terminal input all
    # have to work for this file to be created. No IPC exec shortcut here.
    machine.send_key("alt-ret")
    retry(lambda _: window(machine, "vm-terminal") is not None)
    machine.send_chars("printf vm-keyboard > /tmp/keyboard-worked\n")
    machine.wait_for_file("/tmp/keyboard-worked")
    assert machine.succeed("cat /tmp/keyboard-worked") == "vm-keyboard"
    first = window(machine, "vm-terminal")
    assert first["shell"] == "xdg_shell" and first["visible"], first
    machine.send_key("alt-2")
    retry(lambda _: any(w["name"] == "2" and w["focused"] for w in ipc(machine, kind="get_workspaces")))
    machine.send_key("alt-1")

    ipc(machine, "exec foot -c /etc/foot/foot.ini --app-id vm-second --title vm-second sleep 600")
    retry(lambda _: window(machine, "vm-second") is not None)
    ipc(machine, "exec xterm -class vm-x11 -title vm-x11 -e sleep 600")
    retry(lambda _: window(machine, "vm-x11") is not None)
    assert window(machine, "vm-x11")["shell"] == "xwayland"
    user(machine, "WAYLAND_DISPLAY=wayland-1 wayland-info > /tmp/wayland-info")
    machine.copy_from_machine("/tmp/wayland-info", machine.name)
    machine.screenshot(f"{machine.name}-wayland-and-x11")

    ipc(machine, '[app_id="vm-terminal"] focus')
    ipc(machine, "mark vm-first; floating enable; resize set width 500 px height 300 px; move position 70 px 80 px")
    retry(lambda _: window(machine, "vm-terminal")["rect"]["width"] == 500)
    floating = window(machine, "vm-terminal")
    assert floating["floating"] == "user_on" and "vm-first" in floating["marks"]
    ipc(machine, "fullscreen enable")
    retry(lambda _: window(machine, "vm-terminal")["fullscreen_mode"] == 1)
    machine.screenshot(f"{machine.name}-fullscreen")
    ipc(machine, "fullscreen disable; move scratchpad")
    retry(lambda _: not window(machine, "vm-terminal")["visible"])
    ipc(machine, "scratchpad show; floating disable")
    retry(lambda _: window(machine, "vm-terminal")["visible"])
    ipc(machine, "layout tabbed; gaps inner all set 8; border pixel 3; opacity 0.8")
    assert ipc(machine, kind="get_marks") == ["vm-first"]

    # A real user D-Bus verifies the tray host/watcher's registration too.
    registered = user(machine, "busctl --user get-property org.kde.StatusNotifierWatcher /StatusNotifierWatcher org.kde.StatusNotifierWatcher IsStatusNotifierHostRegistered")
    assert registered.strip() == "b true", registered
    ipc(machine, "exec swaynag -m 'VM translation test' -b Dismiss true")
    machine.wait_until_succeeds("pgrep -x swaynag")
    machine.sleep(1)
    machine.succeed("pgrep -x swaynag")
    machine.screenshot(f"{machine.name}-bar-and-nag")
    machine.succeed("pkill -x swaynag")

    # Save a deterministic subset of state for a C/Rust comparison. Process
    # IDs and wlroots' random foreign-toplevel handles differ between guests.
    state = {
        "bar": ipc(machine, "vm", "get_bar_config"),
        "windows": sorted(
            [(n["name"], n.get("app_id"), n["shell"], n["border"], n["marks"], n["fullscreen_mode"])
             for n in walk(ipc(machine, kind="get_tree")) if "shell" in n],
            key=lambda item: item[0],
        ),
    }
    ipc(machine, "reload")
    retry(lambda _: window(machine, "vm-terminal") is not None)
    ipc(machine, '[class="vm-x11"] kill')
    retry(lambda _: window(machine, "vm-x11") is None)
    # Exit closes the IPC connection before a reply; inspect the compositor's
    # exit status through its login shell instead.
    machine.execute("su - alice -c 'SWAYSOCK=/tmp/sway-ipc.sock swaymsg exit'")
    machine.wait_for_file("/tmp/sway-exit-status")
    assert machine.succeed("cat /tmp/sway-exit-status").strip() == "0"
    machine.copy_from_machine("/tmp/sway.log", machine.name)
    machine.copy_from_machine("/tmp/sway-validate.log", machine.name)
    return state


start_all()
reference_state = exercise(reference)
translated_state = exercise(translated)
assert reference_state == translated_state, (reference_state, translated_state)
