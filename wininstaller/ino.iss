[Setup]
AppName=Wcalc
AppVersion=1.5.0
DefaultDirName={pf}\Wcalc
DefaultGroupName=Wcalc
OutputBaseFilename=Installer

[Files]
Source: "wcalc.exe"; DestDir: "{app}"
Source: "lib\*"; DestDir: "{app}"

[Icons]
Name: "{commondesktop}\wcalc"; Filename: "{app}\wcalc.exe"

