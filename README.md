# Solana Speedsters

![Solana Speedsters Banner](https://via.placeholder.com/1200x630/0d1117/c9d1d9?text=Solana+Speedsters )

**Solana Speedsters** é um jogo de corrida NFT (Play-to-Earn) construído na blockchain Solana. Colecione, aprimore e compita com seus carros NFT para ganhar recompensas em um ecossistema focado em sustentabilidade, transparência e diversão.

**Aviso Importante:** Este projeto está em desenvolvimento. O código não foi auditado. **Não executar qualquer ação de captação ou marketing que busque manipular preços; priorizar transparência, auditoria e segurança.**

---

## 🏁 Visão Geral do Projeto

*   **Tecnologia:** Solana, Anchor, Rust, Metaplex, Next.js, Phaser.
*   **Gênero:** Corrida, Estratégia, Colecionável.
*   **Economia:** Tokens SPL (`$SPEED` para utilidade, `$GOV` para governança).
*   **Lançamento:** Fair Launch via Metaplex Candy Machine.

Consulte o [Whitepaper (link a ser adicionado)] para mais detalhes sobre a visão, mecânicas e tokenomics.

## 🚀 Estrutura do Repositório (Monorepo)

Este repositório utiliza `pnpm workspaces` para gerenciar múltiplos pacotes e aplicações.

*   `apps/web`: O frontend principal em Next.js. Inclui o site institucional, o dashboard do jogador, o marketplace e a interface de mint.
*   `apps/game-client`: O cliente do jogo em Phaser, responsável pela visualização das corridas.
*   `packages/anchor-contracts`: Os smart contracts (programs) em Rust/Anchor que formam a espinha dorsal do jogo na blockchain.
*   `packages/ts-sdk`: Um SDK em TypeScript para facilitar a comunicação entre o frontend e os contratos Anchor.
*   `scripts/`: Scripts utilitários para deploy, gerenciamento da Candy Machine e testes.

## 🛠️ Começando (Ambiente de Desenvolvimento)

### Pré-requisitos

1.  **Node.js** (v18 ou superior)
2.  **pnpm** (`npm install -g pnpm`)
3.  **Rust & Cargo** (`curl https://sh.rustup.rs -sSf | sh` )
4.  **Solana Tool Suite** (`sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install )"`)
5.  **Anchor** (`avm install latest && avm use latest`)
6.  **Docker** (para testes locais com o validador da Solana)

### Instalação

1.  **Clone o repositório:**
    ```bash
    git clone [URL_DO_SEU_REPOSITORIO]
    cd solana-speedsters
    ```

2.  **Instale as dependências:**
    ```bash
    pnpm install
    ```

### Comandos Principais

*   **Construir Contratos:**
    ```bash
    pnpm --filter anchor-contracts build
    ```

*   **Testar Contratos:**
    ```bash
    pnpm --filter anchor-contracts test
    ```

*   **Iniciar Frontend (Modo de Desenvolvimento):**
    ```bash
    pnpm --filter web dev
    ```

*   **Iniciar Cliente do Jogo (Modo de Desenvolvimento):**
    ```bash
    pnpm --filter game-client dev
    ```

## 📜 Contratos On-Chain

Localizados em `packages/anchor-contracts`, os programas incluem:

*   **Contrato de Jogo:** Gerencia a lógica principal das corridas e recompensas.
*   **Contrato de Marketplace:** Permite a negociação de NFTs de carros.
*   **Contrato de Economia:** Lida com staking e vesting de tokens.

## 🔗 Links Úteis

*   **Website:** (Ainda não disponível)
*   **Documentação:** (Ainda não disponível)
*   **Auditoria de Segurança:** (Pendente)

## 🤝 Contribuições

Estamos abertos a contribuições da comunidade! Por favor, leia nosso guia de contribuição (a ser criado) e siga o código de conduta.

---
*Este projeto é fornecido como está, sem garantias. Use por sua conta e risco.*
