# shadow-rs 🦀

![Rust](https://img.shields.io/badge/made%20with-Rust-red)
![Platform](https://img.shields.io/badge/platform-windows-blueviolet)
![Forks](https://img.shields.io/github/forks/joaoviictorti/shadow-rs)
![Stars](https://img.shields.io/github/stars/joaoviictorti/shadow-rs)
![License](https://img.shields.io/github/license/joaoviictorti/shadow-rs)

`shadow-rs` is a Windows kernel rootkit written in Rust, demonstrating advanced techniques for kernel manipulation while leveraging Rust’s safety and performance features. This project is intended for educational and research purposes.

The project also provides useful crates for developing rootkits, such as [**shadowx**](/crates/shadowx/), which consolidates core logic and essential techniques. It includes rootkit-specific tricks, with plans for additional features in future updates.

The documentation on how to execute CLI commands can be found on the [**Wiki**](https://github.com/joaoviictorti/shadow-rs/wiki)

## Table of Contents

* [Legal notice](#legal-notice)
* [Features](#features)
* [Installation](#installation)
* [Supported Platforms](#supported-Platforms)
* [Build Instructions](#build-instructions)
  * [Driver](#driver)
  * [Client](#client)
* [Setup Instructions](#setup-instructions)
  * [Enable Test Mode](#enable-test-mode)
  * [Debug via Windbg](#debug-via-windbg)
  * [Create/Start Service](#createstart-service)
* [Contributing to shadow-rs](#contributing-to-shadow-rs)
* [References](#references)
* [License](#license)

## Legal Notice

> [!IMPORTANT]  
> This project is under development.
> This project is for educational and research purposes. Malicious use of the software is strictly prohibited and discouraged. I am not responsible for any damage caused by improper use of the software.

## Features
 
### Process
- ✅ Process (Hide / Unhide)
- ✅ Process Signature (PP / PPL)
- ✅ Process Protection (Anti-Kill / Dumping)
- ✅ Elevate Process to System
- ✅ Terminate Process
- ✅  List protected and hidden processes

### Thread
- ✅ Thread (Hide / Unhide)
- ✅ Thread Protection (Anti-Kill)
- ✅  List protected and hidden threads

### Driver
- ✅ Driver (Hide / Unhide)
- ✅ Enumerate Driver
- ✅ Driver Signature Enforcement (Enable / Disable)

### Callback
- ✅ List / Remove / Restore Callbacks
    - PsSetCreateProcessNotifyRoutine
    - PsSetCreateThreadNotifyRoutine
    - PsSetLoadImageNotifyRoutine
    - CmRegisterCallbackEx
   - ObRegisterCallbacks (PsProcessType / PsThreadType)
- ✅ List removed callbacks

### Keylogger and Ports
- ✅ Enable Keylogger
- ✅ Ports (Hide / Unhide)
  
### Module and Registry
- ✅ Hide Module
- ✅ Enumerate Module
- ✅ Key and Values (Hide / Unhide)
- ✅ Registry Protection (Anti-Deletion and Overwriting)

### User Mode Code Execution
- ✅ Injection via ZwCreateThreadEx (Shellcode / DLL)
- ✅ APC Injection (Shellcode)


## Installation
- Install Rust from [**here**](https://www.rust-lang.org/learn/get-started).
- Then follow the instructions provided by [**microsoft**](https://github.com/microsoft/windows-drivers-rs?tab=readme-ov-file#getting-started)

## Supported Platforms
- ✅ Windows 10 / 11 (x64)

## Build Instructions

To build the project, ensure you have the Rust toolchain installed. 

#### Driver
To build the driver, first go to the `driver` folder and then run the following command (When you do the first build you have to be as administrator, but after that you won't need to):
```cmd
cargo make default --release
```

This driver can be mapped using `kdmapper` among other exploit tools, for example, to put mapping support, use the command:
```cmd
cargo make default --release --features mapper
```

#### Client
To build the client, first go into the `client` folder, then run the following command:
```cmd
cargo build --release
```

Since some features of the rootkit are not supported due to the controller mapping, use the following command to build the client with only the commands that can be executed with the mapping:
```cmd
cargo build --release --features mapper
```

## Setup Instructions

#### Enable Test Mode or Test Signing Mode 

```cmd
bcdedit /set testsigning on
```

#### [Optional] Debug via Windbg

```cmd
bcdedit /debug on
bcdedit /dbgsettings net hostip:<IP> port:<PORT>
```

#### Create / Start Service

You can use [Service Control Manager](https://docs.microsoft.com/en-us/windows/win32/services/service-control-manager) or [OSR Driver Loader](https://www.osronline.com/article.cfm%5Earticle=157.htm) to load your driver.

## Contributing to shadow-rs
To contribute to shadow-rs, follow these steps:

1. Fork this repository.
2. Create a branch: ```git checkout -b <branch_name>```.
3. Make your changes and confirm them: ```git commit -m '<commit_message>'```.
4. Send to the original branch: ```git push origin <project_name> / <local>```.
5. Create the pull request.

Alternatively, consult the GitHub documentation on how to create a pull request.

## References

- https://synzack.github.io/Blinding-EDR-On-Windows/
- https://leanpub.com/windowskernelprogrammingsecondedition
- https://www.amazon.com/Rootkits-Bootkits-Reversing-Malware-Generation/dp/1593277164
- https://www.amazon.com.br/Rootkits-Subverting-Windows-Greg-Hoglund/dp/0321294319
- https://www.youtube.com/watch?v=t7Rx3crobZU&pp=ugMICgJwdBABGAHKBRBibGFja2hhdCByb290a2l0
- https://imphash.medium.com/windows-process-internals-a-few-concepts-to-know-before-jumping-on-memory-forensics-part-4-16c47b89e826
- https://www.amazon.com.br/Rootkit-Arsenal-Escape-Evasion-Corners/dp/144962636X
- https://github.com/memN0ps/eagle-rs
- https://github.com/mirror/reactos
- https://github.com/Kharos102/ReadWriteDriverSample
- https://github.com/Idov31/Nidhogg
- https://github.com/eversinc33/Banshee
- https://github.com/JKornev/hidden
- https://www.unknowncheats.me/

## License

This project is licensed under the MIT License. See the [LICENSE](/LICENSE) file for details.
