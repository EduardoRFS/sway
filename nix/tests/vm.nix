{ pkgs, packages }:
let
  desktop = package: { ... }: {
    users.users.alice = {
      isNormalUser = true;
      uid = 1000;
      password = "test";
      extraGroups = [ "video" "input" ];
    };
    services.getty.autologinUser = "alice";
    programs.sway = {
      enable = true;
      package = null;
      extraPackages = [ ];
    };
    hardware.graphics.enable = true;
    fonts.packages = [ pkgs.dejavu_fonts ];
    environment.systemPackages = with pkgs; [
      package foot xterm xwayland wayland-utils grim jq swaybg
    ];
    environment.etc."foot/foot.ini".text = ''
      [main]
      font=DejaVu Sans Mono:size=10
      locked-title=yes
    '';
    environment.etc."sway/vm.conf".text = ''
      set $mod Mod1
      font DejaVu Sans Mono 10
      output * bg #334455 solid_color
      output Virtual-1 mode 1280x800 position 0 0
      output Virtual-2 disable
      input * xkb_layout us
      bindsym $mod+Return exec foot -c /etc/foot/foot.ini --app-id vm-terminal --title vm-terminal
      bindsym $mod+2 workspace 2
      bindsym $mod+1 workspace 1
      focus_follows_mouse no
      bar {
        id vm
        status_command while :; do echo vm-status; sleep 1; done
      }
    '';
    programs.bash.loginShellInit = ''
      if [ "$(tty)" = /dev/tty1 ] && [ ! -e /tmp/sway-started ]; then
        touch /tmp/sway-started
        export SWAYSOCK=/tmp/sway-ipc.sock
        export WLR_RENDERER=pixman
        export DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus
        sway --validate -c /etc/sway/vm.conf > /tmp/sway-validate.log 2>&1
        sway -c /etc/sway/vm.conf > /tmp/sway.log 2>&1
        echo $? > /tmp/sway-exit-status
      fi
    '';
    virtualisation = {
      memorySize = 1536;
      cores = 2;
      qemu.options = [ "-vga none -device virtio-gpu-pci" ];
    };
  };
in pkgs.testers.runNixOSTest {
  name = "sway-rust-vm";
  skipTypeCheck = true;
  nodes.reference = desktop packages.reference;
  nodes.translated = desktop packages.sway;
  testScript = builtins.readFile ./vm.py;
}
