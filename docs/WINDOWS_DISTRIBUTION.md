# Distribuição Windows e assinatura

A aplicação1.6 foi homologada em janela real, GPU e testes. O assistente Inno mantém logo, licença, diretório, atalhos e registro por usuário. A execução offline não exige rede. O pacote ainda não possui assinatura Authenticode própria.

## Resultado observado1.6

QA16-pdf instalou em target/installation qa16-pdf, identidade QA separada. Log target/installer-qa16-install.log terminou com sucesso. Registro DisplayVersion1.6.0, atalhos, hashes do executável/documentos e autoteste instalado passaram. Os arquivos keep-me.svg e docs/user-notes.txt foram criados para testar preservação.

Ao executar unins000.exe, Windows App Control recusou a carga. Eventos3033/3077 e3089 de17/10/2026 às19:08:26 registraram requisito de assinatura/política, ID0283ac0f-fff1-49ae-ada1-8a933130cad6. Get-AuthenticodeSignature informou NotSigned. Não houve mudança de política, liberação de arquivo ou tentativa de contornar o bloqueio.

Desinstalação/preservação pós-desinstalação1.6 não foram verificadas. A identidade QA, pasta e atalhos permaneceram; não representam uma segunda instalação de produção.1.4/1.5 tiveram ciclos completos aprovados anteriormente. O sucesso anterior não substitui o ensaio1.6 reprovado.

## Trabalho de publicação pendente

Assinar aplicação, setup e desinstalador com identidade de publicação autorizada, verificar cadeia/timestamp e testar na política de destino. Inno oferece SignTool/SignedUninstaller; nenhuma credencial/certificado foi provisionado ou solicitado neste ciclo. Não criar certificado autoassinado como alegação de confiança pública e não reduzir políticas Windows para aprovar QA. Uma assinatura válida ainda precisa ser aceita pela política do dispositivo.

Depois de disponibilizada uma assinatura aceita, repetir o ciclo completo usando RunId novo e tratar a instalação QA remanescente por desinstalação normal. Não remover seu registro manualmente para esconder falha. O script tradicional ainda para se encontrar identidade QA anterior.

## Conferência independente do payload

installer/test-payload.ps1 usa o TESTMODE existente, em pasta target inédita, sem registro/atalhos/desinstalador. Confere hashes de todos os arquivos, roda self-test e verifica que registros anteriores não mudaram. Isso testa cópia/execução do pacote final; não comprova desinstalação nem resolve o bloqueio.

Referências: [SignedUninstaller no Inno](https://jrsoftware.org/ishelp/topic_setup_signeduninstaller.htm), [diagnóstico App Control](https://learn.microsoft.com/en-us/windows/security/application-security/application-control/app-control-for-business/operations/appcontrol-debugging-and-troubleshooting) e [assinatura e políticas](https://learn.microsoft.com/en-us/windows/security/application-security/application-control/app-control-for-business/deployment/use-code-signing-for-better-control-and-protection).

Resultado16-final-payload: PASS20 arquivos, self-test e snapshots de registros intactos; sem atalhos ou uninstaller. Dados em target/payload-16-final-payload-report.json. Pasta de payload retida para inspeção. O ciclo de instalação completo continua parcial.

## Resultado1.7

Aplicação final:79 unitários,101 checks GPU/121 frames,3 recovery, self-test e desktop canônico/intercalado PASS. TESTMODE17-reverse-payload passou20 hashes e self-test1.7 do executável copiado; registros anteriores intactos e nenhum atalho/desinstalador criado. Evidência target/payload-17-reverse-payload-report.json. A reconferência com os documentos consolidados usa RunId17-reverse-payload-final e mantém o mesmo escopo limitado. Nenhuma nova tentativa de desinstalação bloqueada foi feita.

## Correção1.7.1

81 unitários/107 GPU126frames/3 recuperação/self-test e desktop de atalhos/SaveAs/XML/reabrir PASS. Payload TESTMODE171-slots-payload PASS20 hashes e self-test1.7.1; registros intactos e nenhum atalho/uninstaller. Recurso Windows e metadado SVG conferidos como1.7.1. Reconferência com documentos consolidados usa171-slots-payload-final. O ciclo completo continua parcial pela pendência de assinatura/política já documentada; não houve tentativa de bypass.

## Resultado1.8 —18/10/2026

86 unitários/116 GPU130frames/3 recuperação/self-test e desktop de transferência/menu/SaveAs/XML/reabrir PASS. TESTMODE18-transfers-payload PASS20 hashes e self-test1.8.0 do executável copiado; registros anteriores intactos, nenhum atalho/desinstalador. Recurso Windows1.8.0 e SVG1.8 conferidos. Relatório target/payload-18-transfers-payload-report.json. Reconferência com documentação consolidada usa18-transfers-payload-final. A distribuição completa continua parcial até assinatura/política aceita e ciclo aprovado; não houve nova tentativa de desinstalar a QA1.6 bloqueada.

## Resultado1.9 —18/10/2026

93 unitários/139 checks GPU138frames/3 recuperação/self-test1.9 e desktop de alvo/menu/painel/cancelamento/undo/redo/SaveAs/XML integral/reabrir/fechar PASS. TESTMODE19-relative-payload PASS20 hashes e self-test1.9.0 do exe copiado; registros intactos, nenhum atalho/uninstaller. FileVersion1.9.0.0/ProductVersion1.9.0 e SVG1.9 conferidos. Relatório target/payload-19-relative-payload-report.json. Reconferência com documentos consolidados usa19-relative-payload-final; resultado próprio no relatório/checkpoint. Assinatura aceita e ciclo completo ainda pendentes; nenhuma alteração de política nem tentativa de executar uninstaller bloqueado.
