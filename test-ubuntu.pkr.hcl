source "virtualbox-iso" "ubuntu" {
  guest_os_type = "Ubuntu_64"
  iso_url = "https://releases.ubuntu.com/22.04.3/ubuntu-22.04.3-live-server-amd64.iso"
  iso_checksum = "sha256:a4acfda10b18da50e2ec50ccaf860d7f20b389df8765611142305c0e911d16fd"
  
  ssh_username = "ubuntu"
  ssh_password = "ubuntu"
  ssh_timeout = "30m"
  
  shutdown_command = "echo 'ubuntu' | sudo -S shutdown -P now"
  
  boot_command = [
    "c",
    "linux /casper/vmlinuz --- autoinstall ds='nocloud-net;s=http://{{.HTTPIP}}:{{.HTTPPort}}/'",
    "<enter>",
    "initrd /casper/initrd",
    "<enter>",
    "boot",
    "<enter>"
  ]

  memory = 2048
  cpus = 2
  disk_size = 10000
  
  http_directory = "http"
  
  headless = true
  boot_wait = "5s"
}

build {
  sources = ["source.virtualbox-iso.ubuntu"]
}