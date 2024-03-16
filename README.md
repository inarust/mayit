# mayit
Framework web service berbasiskan [may mini http](https://github.com/Xudong-Huang/may_minihttp)

## Setting Environment

Pastikan sudah melakukan setting environment di komputer atau server nya

```env
MONGOSTRINGAWANGGA=mongodb://localhost:27017
SERVERADDRESS=0.0.0.0:3000
```
pada powershell
```ps
$env:MONGOSTRINGAWANGGA = 'mongodb://localhost:27017'
$env:SERVERADDRESS = '0.0.0.0:3000'
$env:RUST_BACKTRACE = 'full'
dir env:
```