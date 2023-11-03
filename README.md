# rscan

A simple, modular, asynchronous network enumeration tool built with rust, tokio and clap.

## Sample Output

```

+------------+-------------+
| Default Gateway(s) |
+============+=============+
| IP Address | 192.168.1.1 |
+------------+-------------+

+------------+-------------+
| DNS Server |
+============+=============+
| DNS Server | 192.168.1.1 |
+------------+-------------+

+---------------------------+--------+-------------------+-----------+------------------------------+
| IP Address | Device | MAC Address | State | Hostname |
+===========================+========+===================+===========+==============================+
| 169.254.169.254 | dev | | FAILED | N/A |
+---------------------------+--------+-------------------+-----------+------------------------------+
| 192.168.1.1 | dev | 7a:45:58:86:90:ea | REACHABLE | unifi.localdomain. |
+---------------------------+--------+-------------------+-----------+------------------------------+
| 192.168.1.113 | dev | 70:85:c2:24:f3:d7 | STALE | DESKTOP-ISDTPGF.localdomain. |
+---------------------------+--------+-------------------+-----------+------------------------------+
| 192.168.1.228 | dev | 1c:f2:9a:66:66:15 | REACHABLE | N/A |
+---------------------------+--------+-------------------+-----------+------------------------------+
| fe80::7f28:95c9:456c:5087 | dev | 7c:50:79:ea:a6:e1 | STALE | N/A |
+---------------------------+--------+-------------------+-----------+------------------------------+
| fe80::18bc:6db7:caaf:4e66 | dev | 82:2a:ca:18:50:61 | STALE | N/A |
+---------------------------+--------+-------------------+-----------+------------------------------+

+-----------+-----------------------+-------------+-----------------------+------------------------------------------+-----------------------+------------------+----------------------+
| Interface | Chassis ID | System Name | System Description | Management IP(s) | Port ID | Port Description | Capabilities |
+-----------+-----------------------+-------------+-----------------------+------------------------------------------+-----------------------+------------------+----------------------+
| wlp4s0 | mac 78:45:58:4d:25:b4 | SecondFloor | U6-Lite, 6.5.64.14808 | 192.168.1.242, fe80::7a45:58ff:fe4d:25b4 | mac 78:45:58:4d:25:b6 | rai0 | Bridge, Router, Wlan |
+-----------+-----------------------+-------------+-----------------------+------------------------------------------+-----------------------+------------------+----------------------+

```