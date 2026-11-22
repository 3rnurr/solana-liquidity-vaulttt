# Liquidity Vault — Неделя 8 (Каркас)

Каркас программы `liquidity_vault` на Solana (Anchor, Devnet). На этом этапе реализована структура аккаунтов и их инициализация без перевода средств.

## Адреса в Solana Explorer (Devnet)

- Program ID: `9gBkptN51e1vf9VvjcQVqxobhWdcozcQ8Jbezhh6jYxb` — [ссылка на Explorer](https://explorer.solana.com/address/9gBkptN51e1vf9VvjcQVqxobhWdcozcQ8Jbezhh6jYxb?cluster=devnet)
- VaultConfig PDA: `6UD62SwCuyrzFh7VtfGBHgRT8RGtVSfi1p4ZNR5q5ENR` — [ссылка на Explorer](https://explorer.solana.com/address/6UD62SwCuyrzFh7VtfGBHgRT8RGtVSfi1p4ZNR5q5ENR?cluster=devnet)
- Share Mint PDA: `5Uwy9odFiRuK5J2qXn3ZAoCvULvmgn4ubYUvkqQAkXNe` — [ссылка на Explorer](https://explorer.solana.com/address/5Uwy9odFiRuK5J2qXn3ZAoCvULvmgn4ubYUvkqQAkXNe?cluster=devnet)
- Vault Token PDA: `6pJAnBPDvaYUh99wSms7yYHrizHAWAFjoaBMBea1fheE` — [ссылка на Explorer](https://explorer.solana.com/address/6pJAnBPDvaYUh99wSms7yYHrizHAWAFjoaBMBea1fheE?cluster=devnet)
- UserPosition PDA: `9Pw2HxPWsnuroG25Pdm6g4j2jazJbCuDZXNP4kQ1uy8t` — [ссылка на Explorer](https://explorer.solana.com/address/9Pw2HxPWsnuroG25Pdm6g4j2jazJbCuDZXNP4kQ1uy8t?cluster=devnet)

## Схема аккаунтов и владение

Программа не хранит состояние внутри себя — данные и токены лежат в отдельных аккаунтах, часть из которых вычисляется как **PDA (Program Derived Address)**. У PDA нет приватного ключа, поэтому подписывать действия от имени PDA может только сама программа через `signer_seeds`.

- **VaultConfig (PDA, seeds: `["vault_config", asset_mint]`)**
  - Назначение: хранит общие настройки хранилища и учёт активов.
  - Поля: `authority`, `asset_mint`, `share_mint`, `vault_token_account`, `total_assets`, `bump`.
  - Кто управляет/владеет: программа через PDA; администратор (`authority`) задаётся при инициализации.
- **Share Mint (PDA, seeds: `["share_mint", vault_config]`)**
  - Назначение: mint-аккаунт токена-квитанции (доли в пуле).
  - Кто управляет/владеет: `mint::authority` закреплён за PDA `vault_config`, поэтому выпускать (и в будущем сжигать/управлять по правилам контракта) может программа, а не человек.
- **Vault Token Account (PDA, seeds: `["vault_token", vault_config]`)**
  - Назначение: токен-аккаунт, в котором будут храниться внесённые базовые токены (`asset_mint`).
  - Кто управляет/владеет: `token::authority` закреплён за PDA `vault_config`, чтобы никто (включая администратора) не мог забрать токены в обход логики программы.
- **UserPosition (PDA, seeds: `["user_pos", vault_config, user]`)**
  - Назначение: персональная «карточка» пользователя в конкретном vault. Адрес детерминированно вычисляется из `vault_config` и адреса пользователя, поэтому отдельный список всех пользователей хранить не нужно.
  - Поля: `owner`, `vault`, `deposited_amount`, `bump`.
