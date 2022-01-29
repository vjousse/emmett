---
title: Setting up a low-tech server
slug: setting-up-a-low-tech-server
date: 2015-04-26 20:09:01+02:00
tags: archlinux
category: 
link: 
description: 
type: text
status: draft
---


Download Armbian buster (Debian 10) or Armbian bionic (Ubuntu) on https://www.armbian.com/olimex-lime-2/#kernels-archive-all.
Unzip the 7zip archive.
Use https://www.balena.io/etcher/ to flash it on your SD card (mine is a 64Gb X card)

Put in your olimex a20 with a screen, a keyboard and wait for the login screen.

User: root
password: 1234

Change it to whatever you want and creat your first user (who will have sudo access).

  apt install nginx
