# Nscript Lib
this lib contains the core functionality of the nscript runtime.
it contains the full source which i wrote from scratch!
this lib on its own doesnt execute , to use it in your project you can check out
## whats Nscript and why ?
Nscript is a custom minimalistic c like scripting syntax it runs on almost anything with a chip!
if its ARM or x86 or x64, hardware tested as low as a raspberry pi zero!
nscripts doesnt compile, it has its own bytecode like preproccesing system.

Nscript offers a cross platform scripting syntax ( like python )
- typeless , stripped syntaxis
- runtime function and class redeclaration ability
    nscript is capable to update its functions and class code at runtime! no reboot required
- comes with a object oriented and asyncronic approach.
- nameid managed coroutines and name reference constructors in runtime
- thread abilities with buildin channels
- offers a simple multithreaded http server with .nc extention execution on request, server rendered code with html string returns as a site, no need for php!
- able to call system programs and work with other binaries
    this could call bash, to execute a print job, or even to run other code like go, python etc.
    able to wait for the calls output and retrieve it as a variable.
- very easy to use class/object system with managed function registers and propery indexes.
    pre-evaluation/ reflection on setting and calling class related syntaxes
    json to object conversions !
    inherented function register and methods to obtain them as vectors
    property index , objects keep track of the set props, can be obtained as a vector.
    custom (.njh) object to file outputs and inputs.
- very light runtime envoirement! absolutely barerust
- custom written cypher encrytion systems.
- .njh (ini like) filesystem which has a high coassistance with the class system.
- TCP / UDP networking functionality.


## standalone binary
https://github.com/NickJasonHagen/nscriptv3_bin
here you will find the standalone binary.
it will explain on src/main.rs how you can extent the functions and insert your own structs

## rendering
so as another project im building this nscript based game engine.
this allows nscript to render to a canvas using DirectX12, OpenGL , Vulkan or Metal depanding on the OS.
this is tested to work on Linux and Windows.
if you like to see how nscript runs with wgpu , i have a project based on Elhams: blue_engine

https://github.com/NickJasonHagen/blueengine_nscriptv3

Im sharing work on the blueengine discord, if you like to keep track of the progress or to see Elhams epic work join us there!
Blueengine discord (Elham):
https://discord.gg/s3QvNyVwcw
## contact
if you got suggestions / questions or looking to connect, feel free to hit me up!
let me know you wanna talk nscript or anything else!
discord  : @skormaka
telegram : @Skormaka

## shared scripts and shell behaviour
the binary on the bin projects uses the ~/nscript folder and will run the init.nc inside that.
this is only prevented if the first argument on the launch isnt containing a .nc extention
i wrote some CLI tools upon this as you can call these functions in any folder (linux)
and the workingdir will be whereever you are! so this allows to automate

you can find the source of the whole shell @
https://github.com/NickJasonHagen/nc_shell

in your profile/bashrc you can add the line
```bash
export NSCRIPT_PATH="/home/user/.nscript/"
```
this will be set to the nscript macro
@nscriptpath
