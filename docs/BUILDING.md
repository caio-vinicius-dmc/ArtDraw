# Compilar, testar e empacotar

Alvo: Windows 10/11 x64. O editor usa Rust, egui0.31.1 e Vello0.5.1/wgpu24. Assets e Cargo.lock estão versionados; caches/toolchains e binários ficam fora do Git. Linux/macOS e outras arquiteturas não são alvos homologados.

## Caminho padrão: Windows MSVC

1. Instale [Git](https://git-scm.com/downloads/win), [Rust com rustup](https://www.rust-lang.org/tools/install) e [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio).
2. Nos Build Tools, selecione Desktop development with C++ com compilador/linker x64 e Windows SDK.
3. Use PowerShell. Se o linker não for encontrado, abra Developer PowerShell for VS. Não desative políticas da máquina para executar scripts; em ambientes gerenciados, use um terminal/política aprovados.

```powershell
git clone https://github.com/caio-vinicius-dmc/ArtDraw.git
cd ArtDraw
rustup default stable-x86_64-pc-windows-msvc
cargo fetch --locked
.\build.ps1 -Mode All -Offline
```

O script aceita Check, Test, Release, All e Environment. Sem -Offline, Cargo pode obter dependências; com -Offline exige cache previamente preparado. Sempre usa --locked. Os recursos Windows são compilados pelo rc.exe do SDK; build.rs procura as versões instaladas em `Windows Kits/10/bin/<versão>/x64/rc.exe` ou usa ARTDRAW_RC/PATH. A seleção efetiva de versão do SDK considera os diretórios versionados. [Sintaxe oficial de RC](https://learn.microsoft.com/en-us/windows/win32/menurc/using-rc-the-rc-command-line-).

Saída padrão: target/release/coreldraw_clone.exe. Release usa opt3, ThinLTO, um codegen unit e strip. Build não altera documentos do usuário.

Se preferir executar Cargo diretamente em ambiente configurado:

```powershell
.\scripts\update-parity.ps1 -Check
cargo check --locked
cargo test --locked
cargo build --release --locked
```

## GNU/MinGW

Também existe o caminho x86_64-pc-windows-gnu, usado no binário experimental1.9.0. Instale Rust GNU e uma distribuição MinGW compatível que forneça GCC, binutils/windres e import libs Windows. ARTDRAW_WINDRES pode apontar explicitamente para windres. Não misture CRTs de distribuições distintas.

A pasta .tools, se existir, ativa a configuração histórica de toolchain local: Rust GNU, LLVM assembler e bridges próprios. Ela não é distribuída no Git nem necessária ao MSVC. O script só ativa esses bridges em host GNU. Detalhes de linker/CRT em ARCHITECTURE_MEMORY.md. Não há promessa de que toda combinação Rust/MinGW funcione sem configuração.

## Verificações e evidência

```powershell
.\scripts\update-parity.ps1 -Check
.\build.ps1 -Mode Check -Offline
.\build.ps1 -Mode Test -Offline
.\build.ps1 -Mode Release -Offline
New-Item -ItemType Directory -Force target/qa | Out-Null
Start-Process .\target\release\coreldraw_clone.exe -ArgumentList '--self-test', 'target/qa/self-test.txt' -Wait
Start-Process .\target\release\coreldraw_clone.exe -ArgumentList '--interaction-smoke', 'target/qa/interaction.png' -Wait
```

O self-test não precisa abrir a interface. Interaction-smoke abre a aplicação, injeta eventos, renderiza com Vello, grava relatório/PNG/fixtures e encerra. Requer sessão Windows/GPU; não se presume renderização disponível em CI. O relatório fica ao lado do PNG com extensão .txt; confirme PASS e código de saída, não apenas existência do arquivo. Há também --gpu-smoke, --recovery-smoke e os modos PDF descritos em PDF_EXPORT.md. Use caminhos exclusivos para cada ensaio e isole recuperação/preferências em target quando testar manualmente.

O CI .github/workflows/windows.yml verifica paridade, cargo check e testes em Windows MSVC. Não instala o aplicativo, não executa QA de GPU nem certifica compatibilidade CDR. Dependências são baixadas uma vez e verificações seguintes rodam offline. Resultados históricos de outras máquinas estão em VALIDATION.md e não substituem seu teste.

## Empacotar

Depois de um release aprovado, instale [Inno Setup6.7.3](https://jrsoftware.org/isdl.php) e disponibilize ISCC.exe no PATH (ou em .tools/inno). Rode:

```powershell
.\package.ps1
```

Saídas: dist/ArtDraw-Setup.exe, dist/ArtDraw-Portable.zip e dist/SHA256SUMS.txt. O pacote inclui fontes de avisos de licença, documentação e fonte incorporada com sua licença. Nenhum desses artefatos pertence ao histórico Git; publique nos Releases com notas e hashes.

O caminho opcional de gerar novamente os bitmaps da marca usa Python + Pillow em installer/prepare_branding.py; não é necessário para compilar porque os assets estão no repositório. O QA independente de PDF requer Poppler, pypdf, numpy e Pillow; scripts/verify-pdf.py descreve os parâmetros.

Assinatura Authenticode e aceitação por políticas Windows continuam pendentes. Leia WINDOWS_DISTRIBUTION.md antes dos scripts de instalador: o QA completo cria registro/atalhos e uma instalação; exige RunId único e preserva fixtures. O TESTMODE de test-payload.ps1 confere apenas cópia/autoteste sem registro/desinstalador, não substitui um ciclo completo. Nunca altere políticas de segurança para aprovar QA.

## Organização do repositório

- src/ — modelo, geometria, UI, render e formatos.
- tests/fixtures/ — documentos próprios para regressões/migrações.
- assets/ — marca própria, ícone e manifesto necessários ao build.
- docs/ — estudo, lacunas, critérios, fluxos e capturas reais.
- scripts/ — sincronização de paridade/memória e verificador PDF.
- installer/ — Inno Setup e ferramentas auxiliares/históricas.

Caminhos target/, dist/, .tools/, output/ e tmp/ são locais e ignorados. O primeiro fetch precisa de rede; o aplicativo não exige rede para editar.
