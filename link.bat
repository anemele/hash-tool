@echo off

if "%~1" == "" echo where hash? & exit /b

for %%e in (sha256sum.exe, md5sum.exe) do (
    if exist %%e del %%e
    mklink /h %%e %1
)