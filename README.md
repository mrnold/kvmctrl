# kvmctrl
A tool to control KVM switch devices by sending hotkeys through USB gadgetfs.

Devices:
- IOGear GCS1104

Hosts:
- Beagleboard-xM with Arch Linux ARM

Build:
- For Beagleboard-xM: cross build --target armv7-unknown-linux-gnueabihf

Run:
- From the Beagleboard: sudo kvmctrl all --port X

This tool lets me keep my KVM switch on one side of a wall with all the computers, and a keyboard/mouse/monitor on the other side of the wall while only running one set of cables. The Beagleboard USB host port connects to the labeled keyboard port on the KVM switch, and my actual keyboard connects to a generic USB port on the switch.