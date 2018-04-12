# msikeyboard-rs

Rust library to control backlit on laptop keyboards made by MSI. Tested on a MSI GE62 6QD.

As of now, the library supports Normal, Gaming, Breathing and Wave mode. It uses the hidapi-rs crate and is mostly made for linux users.

To run it, you need write-access to the device. An experimental udev rule is placed in the rules folder. You need to place it inside /etc/udev/rules.d folder and run udevadm trigger as root. 

Licensing:

This program is licensed as 3-clause BSD, terms are available in the LICENSE file.

Based heavily on a c++/Qt GUI version of msikeyboard by OliPro007.

Based on a command-line version of msi-keyboard by bparker06.

Based on the gist referenced in the issue #43 of the original project.

Based on a nodejs version of msi-keyboard by Steve Lacy of wearefractal.com:

Copyright (c) 2013 | Steve Lacy slacy.me | Fractal wearefractal.com contact@wearefractal.com

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

$The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
