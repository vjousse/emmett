---
layout: post
title:  Simple disk encryption tutorial with archlinux
excerpt: "Here at AlloMedia, for security reasons, we're encrypting every laptop disk by default. As I'm using archlinux, I went to the wiki to follow how to \"just\" encrypt my disk. And well, the page is a little bit overcrowded, at the very least. Let's clarify that a little bit."
date: "2018-02-01 09:00:00+01:00"
categories: archlinux
tags: archlinux tutorial
slug: simple-disk-encryption-tutorial-with-archlinux
---

We all love [archlinux](https://www.archlinux.org/), or if we don't, we're using Fedora or Debian, and trolling is (almost) out of the scope of this article.

But let's be honest, even if the [wiki](http://wiki.archlinux.org/) is great, it can be intimidating sometimes. That's what happened to me yesterday. At [AlloMedia](http://www.allo-media.net), for security reasons, we're encrypting every laptop disk by default. As I'm using archlinux, I went to the wiki to follow how to "just" encrypt my disk. And well, [the page](https://wiki.archlinux.org/index.php/Disk_encryption) is a little bit overcrowded, at the very least.

<!-- TEASER_END -->

You have first to read about 10 pages of documentation, to learn that you now have to choose between 6 methods (*Loop-AES, dm-crypt +/- LUKS, Truecrypt, eCryptfs, EncFS*) and read every \*#! page to understand which one you may want to choose. I've choosen for you.

## [Lvm](https://en.wikipedia.org/wiki/Logical_Volume_Manager_(Linux)) on [Luks](https://en.wikipedia.org/wiki/Linux_Unified_Key_Setup)

This is shipped with the kernel and seems to be the "default" on other distributions. It totally fits my needs: encrypt the whole system, swap included, and decrypt the system on boot using a passphrase.

If that's what you want to do too, follow the white rabbit, Neo.

## Following the rabbit

We will assume that you can erase your disk and start with a fresh install, if it's not the case, this article may not be for you. For the sake of this article, we will use `/dev/nvme0n1` as the main disk of the laptop. You may have something different like `/dev/sda`, that's fine, just replace `/dev/nvme0n1` by `/dev/sda` in the rest of the article.

First, follow the [Archlinux installation guide](https://wiki.archlinux.org/index.php/Installation_guide) to the point just before __Format the partitions__, where they are telling you to modify the partition tables using __fdisk__ or __parted__. Here, you will need to erase all your partitions and create what's needed for the encryption.

### Clean and safely erase your disk

First, use `fdisk` or `gdisk` (if you're using UEFI) to wipe out what's on your disk, i.e. removing all existing partitions (of course, this will delete all the data on your disk…).

For example, for `gdisk`:
```bash
gdisk /dev/nvme0n1

GPT fdisk (gdisk) version 1.0.3

Partition table scan:
    MBR: protective
    BSD: not present
    APM: not present
    GPT: present

Found valid GPT with protective MBR; using GPT.

Command (? for help):
```

Use `p` to print your partition schema, and `d` to delete partitions. Once it's done, use `w` to write your changes to the disk (that is to say, __again__, deleting all the data on your disk) and quit `gdisk`.

Every page on the archlinux wiki says you should first be sure that no previous data will still be readable on your disk (if you have a new computer with nothing on it, this doesn't apply to you).

So we will put random stuff on our disk to be sure to overwrite everything that may still be on it. You can read the [wiki page](https://wiki.archlinux.org/index.php/Securely_wipe_disk#Random_data) or just run the following command:

    dd if=/dev/urandom > /dev/nvme0n1


### Partitionning

We now have a clean disk, let's create what's needed for our encrypted system, that is to say 2 partitions: a partition for `/boot` (that will not be encrypted) and another one for our encrypted volumes (where we will later put `/` and our `swap`).

Here is what we want to have (output of my `gdisk` with the `p` command):

    Number  Start (sector)    End (sector)  Size       Code  Name
       1            2048         1050623   512.0 MiB   EF00  EFI System
       2         1050624      1000215182   476.4 GiB   8E00  Linux LVM

First, create the partition where `/boot` will be mounted of type `8300` (512Mo is a good size) following the [archlinux wiki](https://wiki.archlinux.org/index.php/EFI_System_Partition#Create_the_partition). I'm assuming you're using a system compatible with UEFI, if it's not the case, you may want to document yourself a little bit more using the wiki. Format the partition using _FAT32_.

    mkfs.fat -F32 /dev/nvme0n1p1

Create the other partition of code `8E00` using the remaining space.

You should now have only 2 partitions, one for `/boot` that will not be encrypted, and another one that you will first encrypt, and then put your volumes on it (`/` and `swap`). In my case, the first partition that will be used for `/boot` is named `/dev/nvme0n1p1`, and the other one `/dev/nvme0n1p2`. You may have something like `/dev/sda1` and `/dev/sda2` if your partition naming scheme is not the same than mine.

You can then follow the [LVM on LUKS section](https://wiki.archlinux.org/index.php/Dm-crypt/Encrypting_an_entire_system#LVM_on_LUKS) section.

I don't like having separate partitions for `/` and `/home`. Every time I've done that, I always regretted the amount of space I allocated for each. So now, I'm only creating one `/` partition with everything inside.

In short, below are the commands you should be running for your encrypted volumes (I'm creating a 8Go swap partition).

Crypt the partition and open it with your key:
```bash
cryptsetup luksFormat --type luks2 /dev/nvme0n1p2
cryptsetup open /dev/nvme0n1p2 cryptolvm
```

Create the LVM volumes on it (swap and root):
```bash
pvcreate /dev/mapper/cryptolvm
vgcreate MyVol /dev/mapper/cryptolvm
lvcreate -L 8G MyVol -n swap
lvcreate -l 100%FREE MyVol -n root
```

Format the root and swap volumes:
```bash
mkfs.ext4 /dev/mapper/MyVol-root
mkswap /dev/mapper/MyVol-swap
```

Mount the file systems:
```bash
mount /dev/mapper/MyVol-root /mnt
swapon /dev/mapper/MyVol-swap
```

The arch wiki tells you to format you boot partition using `ext2`, but for me this was a bad idea, as I want the UEFI manager of my Dell XPS 9550 to be able to boot on my `/boot` partition. So, as I said above, I formatted this partition using `FAT32`.

Mount the `/boot` partition:
```bash
mkdir /mnt/boot
mount /dev/nvme0n1p2 /mnt/boot
```

You can then follow the (`mkinitcpio` part of the archlinux wiki)[https://wiki.archlinux.org/index.php/Dm-crypt/Encrypting_an_entire_system#Configuring_mkinitcpio_2].

Be sure to have something like that in your `mkinitcpio.conf` file:

    HOOKS=(... keyboard keymap block encrypt lvm2 ... filesystems ...)

Then continue to install you system normally. Of course, be sure to configure your grub accordingly to your encrypted setup by [following the wiki](https://wiki.archlinux.org/index.php/Dm-crypt/System_configuration#Boot_loader).

For the record, here is my `/etc/defaults/grub` file (it's used to generate the `/boot/grub/grub.cfg` file by using `grub-mkconfig -o /boot/grub/grub.cfg`):

```bash
# GRUB boot loader configuration

GRUB_DEFAULT=0
GRUB_TIMEOUT=1
GRUB_DISTRIBUTOR="Arch"
GRUB_CMDLINE_LINUX_DEFAULT="resume=/dev/mapper/MyVol-swap nouveau.modeset=0 i915.preliminary_hw_support=1 acpi_backlight=vendor acpi_osi=Linux"
#GRUB_CMDLINE_LINUX_DEFAULT=""
#GRUB_CMDLINE_LINUX=""
GRUB_CMDLINE_LINUX="cryptdevice=/dev/nvme0n1p2:cryptolvm"

GRUB_ENABLE_CRYPTODISK=y

# Preload both GPT and MBR modules so that they are not missed
GRUB_PRELOAD_MODULES="part_gpt part_msdos"

# Uncomment to enable booting from LUKS encrypted devices
#GRUB_ENABLE_CRYPTODISK=y

# Uncomment to enable Hidden Menu, and optionally hide the timeout count
#GRUB_HIDDEN_TIMEOUT=5
#GRUB_HIDDEN_TIMEOUT_QUIET=true

# Uncomment to use basic console
GRUB_TERMINAL_INPUT=console

# Uncomment to disable graphical terminal
#GRUB_TERMINAL_OUTPUT=console

# The resolution used on graphical terminal
# note that you can use only modes which your graphic card supports via VBE
# you can see them in real GRUB with the command `vbeinfo'
GRUB_GFXMODE=auto

# Uncomment to allow the kernel use the same resolution used by grub
GRUB_GFXPAYLOAD_LINUX=keep

# Uncomment if you want GRUB to pass to the Linux kernel the old parameter
# format "root=/dev/xxx" instead of "root=/dev/disk/by-uuid/xxx"
#GRUB_DISABLE_LINUX_UUID=true

# Uncomment to disable generation of recovery mode menu entries
GRUB_DISABLE_RECOVERY=true

# Uncomment and set to the desired menu colors.  Used by normal and wallpaper
# modes only.  Entries specified as foreground/background.
#GRUB_COLOR_NORMAL="light-blue/black"
#GRUB_COLOR_HIGHLIGHT="light-cyan/blue"

# Uncomment one of them for the gfx desired, a image background or a gfxtheme
#GRUB_BACKGROUND="/path/to/wallpaper"
#GRUB_THEME="/path/to/gfxtheme"

# Uncomment to get a beep at GRUB start
#GRUB_INIT_TUNE="480 440 1"

# Uncomment to make GRUB remember the last selection. This requires to
# set 'GRUB_DEFAULT=saved' above.
#GRUB_SAVEDEFAULT="true"
```

Enjoy your encrypted archlinux!
