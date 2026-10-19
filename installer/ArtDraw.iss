; ArtDraw Windows installer. Original product code remains Rust; installer engine: Inno Setup.
#define AppVersion "1.9.0"
[Setup]
#ifdef QA
AppId={{D9281462-263E-4BCE-90CA-567A9FA921BD}-QA
AppName=ArtDraw Homologacao
#else
AppId={{D9281462-263E-4BCE-90CA-567A9FA921BD}
AppName=ArtDraw
#endif
AppVersion={#AppVersion}
AppPublisher=ArtDraw
AppVerName=ArtDraw {#AppVersion}
SourceDir=..
DefaultDirName={localappdata}\Programs\ArtDraw
DefaultGroupName=ArtDraw
DisableWelcomePage=no
DisableDirPage=no
DisableProgramGroupPage=no
AllowNoIcons=yes
UsePreviousAppDir=yes
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0
WizardStyle=modern light
WizardSizePercent=115
LicenseFile=installer\TERMS.txt
SetupIconFile=assets\branding\app.ico
WizardImageFile=assets\branding\wizard.bmp
WizardSmallImageFile=assets\branding\wizard-small.bmp
WizardImageStretch=yes
WizardImageBackColor=$FFFFFF
UninstallDisplayIcon={app}\coreldraw_clone.exe
OutputDir=dist
OutputBaseFilename=ArtDraw-Setup
Compression=lzma2
SolidCompression=yes
CloseApplications=yes
RestartApplications=no
SetupLogging=yes
VersionInfoVersion={#AppVersion}.0
VersionInfoDescription=Instalador do ArtDraw
Uninstallable=not IsTestMode
CreateUninstallRegKey=not IsTestMode

[Languages]
Name: brazilianportuguese; MessagesFile: compiler:Languages\BrazilianPortuguese.isl

[Tasks]
Name: desktopicon; Description: Criar um atalho na área de trabalho; GroupDescription: Atalhos adicionais:; Flags: unchecked; Check: not IsTestMode

[Files]
Source: assets\branding\splash.bmp; Flags: dontcopy
Source: target\release\coreldraw_clone.exe; DestDir: {app}; Flags: ignoreversion
Source: README.md; DestDir: {app}; Flags: ignoreversion
Source: AGENTS.md; DestDir: {app}; Flags: ignoreversion
Source: VALIDATION.md; DestDir: {app}; Flags: ignoreversion
Source: CHANGELOG.md; DestDir: {app}; Flags: ignoreversion
Source: LICENSE; DestDir: {app}; Flags: ignoreversion
Source: installer\TERMS.txt; DestDir: {app}; DestName: TERMS.txt; Flags: ignoreversion
Source: dist\THIRD_PARTY_NOTICES.txt; DestDir: {app}; Flags: ignoreversion
Source: docs\*.md; DestDir: {app}\docs; Flags: ignoreversion
Source: docs\FEATURE_MATRIX.csv; DestDir: {app}\docs; Flags: ignoreversion

[Icons]
Name: {group}\ArtDraw; Filename: {app}\coreldraw_clone.exe; Check: not IsTestMode
Name: {group}\Desinstalar ArtDraw; Filename: {uninstallexe}; Check: not IsTestMode
Name: {autodesktop}\ArtDraw; Filename: {app}\coreldraw_clone.exe; Tasks: desktopicon; Check: not IsTestMode

[Run]
Filename: {app}\coreldraw_clone.exe; Description: Abrir o ArtDraw; Flags: nowait postinstall skipifsilent; Check: not IsTestMode

[Code]
function IsTestMode: Boolean;
begin
  Result := ExpandConstant('{param:TESTMODE|0}') = '1';
end;

procedure InitializeWizard;
var
  Splash: TSetupForm;
  Logo: TBitmapImage;
  Title, Status: TNewStaticText;
  Bar: TNewProgressBar;
  I: Integer;
begin
  if WizardSilent then exit;
  ExtractTemporaryFile('splash.bmp');
  Splash := CreateCustomForm(ScaleX(600), ScaleY(415), False, False);
  try
    Splash.Caption := 'ArtDraw — Preparando instalação';
    Splash.BorderStyle := bsNone;
    Splash.Color := clWhite;
    Splash.Position := poScreenCenter;
    Logo := TBitmapImage.Create(Splash);
    Logo.Parent := Splash;
    Logo.SetBounds(ScaleX(20), ScaleY(0), ScaleX(560), ScaleY(313));
    Logo.Stretch := True;
    Logo.Bitmap.LoadFromFile(ExpandConstant('{tmp}\splash.bmp'));
    Title := TNewStaticText.Create(Splash);
    Title.Parent := Splash;
    Title.AutoSize := False;
    Title.SetBounds(0, ScaleY(298), Splash.ClientWidth, ScaleY(38));
    Title.Alignment := taCenter;
    Title.Font.Size := 24;
    Title.Font.Style := [fsBold];
    Title.Caption := 'ArtDraw';
    Status := TNewStaticText.Create(Splash);
    Status.Parent := Splash;
    Status.AutoSize := False;
    Status.Alignment := taCenter;
    Status.SetBounds(0, ScaleY(347), Splash.ClientWidth, ScaleY(24));
    Status.Caption := 'Preparando o assistente de instalação…';
    Bar := TNewProgressBar.Create(Splash);
    Bar.Parent := Splash;
    Bar.SetBounds(ScaleX(54), ScaleY(383), ScaleX(492), ScaleY(6));
    Bar.Style := npbstMarquee;
    Splash.Show;
    Splash.Update;
    for I := 1 to 16 do begin
      Sleep(70);
      Splash.Update;
    end;
  finally
    Splash.Free;
  end;
end;
