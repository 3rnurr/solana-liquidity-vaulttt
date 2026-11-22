# Liquidity Vault

Сквозной проект курса: DeFi-vault на Solana (Anchor, Devnet), который принимает токены пользователей, выпускает share-токены и размещает средства в пуле ликвидности.

## Прогресс по неделям

| Неделя | Тема | Статус |
| :--- | :--- | :--- |
| 8 | Каркас vault: аккаунт настроек, PDA пользователя, share-токен, хранилище | ✅ |
| 9 | Депозит: пользователь вносит токены и получает share-токены | ⏳ |
| 10 | CPI и вывод: сжигание долей, возврат токенов с подписью PDA | ⏳ |
| 11 | АММ и концентрированная ликвидность: свои токены и CLMM-пул Raydium | ⏳ |
| 12 | Swap токенов vault через CLMM-пул Raydium с `min_amount_out` | ⏳ |
| 13 | Позиция vault в CLMM: открыть в диапазоне, собрать комиссии, закрыть, переставить | ⏳ |
| 14 | Capstone build week: веб-интерфейс и документация | ⏳ |
| 15 | Solana Demo Day: живая демонстрация продукта | ⏳ |

## Адреса в Solana Explorer (Devnet)

- **Program ID:** `9gBkptN51e1vf9VvjcQVqxobhWdcozcQ8Jbezhh6jYxb` — [Explorer](https://explorer.solana.com/address/9gBkptN51e1vf9VvjcQVqxobhWdcozcQ8Jbezhh6jYxb?cluster=devnet)
- **VaultConfig PDA:** `6UD62SwCuyrzFh7VtfGBHgRT8RGtVSfi1p4ZNR5q5ENR` — [Explorer](https://explorer.solana.com/address/6UD62SwCuyrzFh7VtfGBHgRT8RGtVSfi1p4ZNR5q5ENR?cluster=devnet)
- **Share Mint PDA:** `5Uwy9odFiRuK5J2qXn3ZAoCvULvmgn4ubYUvkqQAkXNe` — [Explorer](https://explorer.solana.com/address/5Uwy9odFiRuK5J2qXn3ZAoCvULvmgn4ubYUvkqQAkXNe?cluster=devnet)
- **Vault Token PDA:** `6pJAnBPDvaYUh99wSms7yYHrizHAWAFjoaBMBea1fheE` — [Explorer](https://explorer.solana.com/address/6pJAnBPDvaYUh99wSms7yYHrizHAWAFjoaBMBea1fheE?cluster=devnet)
- **UserPosition PDA:** `9Pw2HxPWsnuroG25Pdm6g4j2jazJbCuDZXNP4kQ1uy8t` — [Explorer](https://explorer.solana.com/address/9Pw2HxPWsnuroG25Pdm6g4j2jazJbCuDZXNP4kQ1uy8t?cluster=devnet)

## Аккаунты и владение

Программа не хранит данные внутри себя — состояние и токены лежат в отдельных аккаунтах. Часть из них является **PDA (Program Derived Address)**: у PDA нет приватного ключа, поэтому подписывать действия от её имени может только программа через `signer_seeds`.

| Компонент | Тип | Seeds | Назначение и кто управляет |
| :--- | :--- | :--- | :--- |
| `VaultConfig` | PDA | `["vault_config", asset_mint]` | Хранит `authority`, `asset_mint`, `share_mint`, `vault_token_account`, `total_assets`, `bump`. Управляется программой |
| `Share Mint` | PDA | `["share_mint", vault_config]` | Mint-аккаунт share-токена (доли в пуле). `mint::authority` — PDA `vault_config` |
| `Vault Token` | PDA | `["vault_token", vault_config]` | Хранилище внесённых базовых токенов. `token::authority` — PDA `vault_config` |
| `UserPosition` | PDA | `["user_pos", vault_config, user]` | Карточка пользователя (`owner`, `vault`, `deposited_amount`, `bump`). Вычисляется по адресу пользователя |

## Безопасность

В репозитории не хранятся файлы ключей (`.json`), приватные ключи и seed-фразы. Все действия выполняются в сети **Devnet**.
