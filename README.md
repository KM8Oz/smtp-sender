# smtp-sender
## nodejs smtp client that bind the 25 port in localhost no smtp server needed.

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