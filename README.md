## smtp-sender: ![GitHub](https://img.shields.io/github/license/KM8Oz/smtp-sender?style=plastic) ![npms.io (quality)](https://img.shields.io/npms-io/quality-score/smtp-sender?style=plastic) ![npm](https://img.shields.io/npm/dw/smtp-sender?style=plastic) ![npm bundle size](https://img.shields.io/bundlephobia/min/smtp-sender?style=plastic)

### nodejs smtp client that bind the 25 port in localhost no smtp server needed.

-----------------------------

> supported envirements:
    > `aarch64-apple-darwin`
    > `aarch64-pc-windows-msvc`
    > `x86_64-apple-darwin`

-----------------------------

> usage: 

`yarn add smtp-sender` or `npm i smtp-sender`

```
import { send } from 'smtp-sender'
send(
username: string,
timestamp: string, 
authtype: string, 
password: string, 
server: string, 
port: number, 
requireTls: boolean, 
from: string, 
to: string, 
replay: string, 
subject: string, 
body: string
): Promise<void>
```