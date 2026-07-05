use sqlx::PgPool;

async fn run_sql(pool: &PgPool, sql: &str) -> Result<(), sqlx::Error> {
    sqlx::query(sql).execute(pool).await?;
    Ok(())
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    run_sql(pool, "CREATE TABLE IF NOT EXISTS users (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        email VARCHAR(255) UNIQUE NOT NULL,
        password_hash VARCHAR(255) NOT NULL,
        role VARCHAR(20) NOT NULL DEFAULT 'user',
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    )").await?;

    run_sql(pool, "CREATE TABLE IF NOT EXISTS wallets (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        user_id UUID NOT NULL REFERENCES users(id),
        currency VARCHAR(10) NOT NULL,
        balance DECIMAL(40, 8) NOT NULL DEFAULT 0,
        locked DECIMAL(40, 8) NOT NULL DEFAULT 0,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        UNIQUE(user_id, currency)
    )").await?;

    run_sql(pool, "CREATE TABLE IF NOT EXISTS orders (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        user_id UUID NOT NULL REFERENCES users(id),
        side VARCHAR(4) NOT NULL,
        order_type VARCHAR(10) NOT NULL,
        base_currency VARCHAR(10) NOT NULL,
        quote_currency VARCHAR(10) NOT NULL,
        price DECIMAL(40, 8),
        quantity DECIMAL(40, 8) NOT NULL,
        filled DECIMAL(40, 8) NOT NULL DEFAULT 0,
        status VARCHAR(20) NOT NULL DEFAULT 'open',
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    )").await?;

    run_sql(pool, "CREATE TABLE IF NOT EXISTS trades (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        buy_order_id UUID NOT NULL REFERENCES orders(id),
        sell_order_id UUID NOT NULL REFERENCES orders(id),
        base_currency VARCHAR(10) NOT NULL,
        quote_currency VARCHAR(10) NOT NULL,
        price DECIMAL(40, 8) NOT NULL,
        quantity DECIMAL(40, 8) NOT NULL,
        total DECIMAL(40, 8) NOT NULL,
        taker_side VARCHAR(4) NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    )").await?;

    run_sql(pool, "CREATE TABLE IF NOT EXISTS manual_transactions (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        user_id UUID NOT NULL REFERENCES users(id),
        tx_type VARCHAR(20) NOT NULL,
        currency VARCHAR(10) NOT NULL DEFAULT 'EGP',
        amount DECIMAL(40, 8) NOT NULL,
        status VARCHAR(20) NOT NULL DEFAULT 'pending',
        admin_id UUID REFERENCES users(id),
        notes TEXT,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    )").await?;

    run_sql(pool, "CREATE TABLE IF NOT EXISTS futures_positions (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        user_id UUID NOT NULL REFERENCES users(id),
        pair VARCHAR(20) NOT NULL,
        side VARCHAR(10) NOT NULL,
        quantity DECIMAL(40, 8) NOT NULL,
        entry_price DECIMAL(40, 8) NOT NULL,
        mark_price DECIMAL(40, 8) NOT NULL,
        liquidation_price DECIMAL(40, 8) NOT NULL,
        leverage INT NOT NULL DEFAULT 1,
        margin DECIMAL(40, 8) NOT NULL DEFAULT 0,
        unrealized_pnl DECIMAL(40, 8) NOT NULL DEFAULT 0,
        realized_pnl DECIMAL(40, 8) NOT NULL DEFAULT 0,
        status VARCHAR(20) NOT NULL DEFAULT 'open',
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    )").await?;

    run_sql(pool, "CREATE TABLE IF NOT EXISTS p2p_offers (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        user_id UUID NOT NULL REFERENCES users(id),
        type VARCHAR(10) NOT NULL,
        currency VARCHAR(10) NOT NULL,
        fiat_currency VARCHAR(10) NOT NULL DEFAULT 'EGP',
        price DECIMAL(40, 8) NOT NULL,
        min_amount DECIMAL(40, 8) NOT NULL,
        max_amount DECIMAL(40, 8) NOT NULL,
        available DECIMAL(40, 8) NOT NULL,
        payment_method VARCHAR(100) NOT NULL DEFAULT 'bank_transfer',
        status VARCHAR(20) NOT NULL DEFAULT 'active',
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    )").await?;

    run_sql(pool, "CREATE TABLE IF NOT EXISTS p2p_orders (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        offer_id UUID NOT NULL REFERENCES p2p_offers(id),
        buyer_id UUID NOT NULL REFERENCES users(id),
        seller_id UUID NOT NULL REFERENCES users(id),
        amount DECIMAL(40, 8) NOT NULL,
        total DECIMAL(40, 8) NOT NULL,
        status VARCHAR(20) NOT NULL DEFAULT 'pending',
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    )").await?;

    run_sql(pool, "CREATE TABLE IF NOT EXISTS p2p_messages (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        order_id UUID NOT NULL REFERENCES p2p_orders(id),
        sender_id UUID NOT NULL REFERENCES users(id),
        message TEXT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    )").await?;

    run_sql(pool, "CREATE TABLE IF NOT EXISTS system_settings (
        key VARCHAR(100) PRIMARY KEY,
        value JSONB NOT NULL DEFAULT '{}',
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    )").await?;

    run_sql(pool, "CREATE TABLE IF NOT EXISTS currencies (
        code VARCHAR(10) PRIMARY KEY,
        name VARCHAR(100) NOT NULL,
        name_ar VARCHAR(100) NOT NULL,
        type VARCHAR(20) NOT NULL DEFAULT 'crypto',
        decimals INT NOT NULL DEFAULT 8,
        withdrawal_fee DECIMAL(40, 8) NOT NULL DEFAULT 0,
        min_withdrawal DECIMAL(40, 8) NOT NULL DEFAULT 0,
        deposit_enabled BOOLEAN NOT NULL DEFAULT true,
        withdrawal_enabled BOOLEAN NOT NULL DEFAULT true,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    )").await?;

    run_sql(pool, "CREATE TABLE IF NOT EXISTS trading_pairs (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        base_currency VARCHAR(10) NOT NULL,
        quote_currency VARCHAR(10) NOT NULL,
        maker_fee DECIMAL(10, 6) NOT NULL DEFAULT 0.001,
        taker_fee DECIMAL(10, 6) NOT NULL DEFAULT 0.001,
        min_trade DECIMAL(40, 8) NOT NULL DEFAULT 0.0001,
        max_leverage INT NOT NULL DEFAULT 1,
        futures_enabled BOOLEAN NOT NULL DEFAULT false,
        spot_enabled BOOLEAN NOT NULL DEFAULT true,
        is_active BOOLEAN NOT NULL DEFAULT true,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        UNIQUE(base_currency, quote_currency)
    )"    ).await?;

    run_sql(pool, "ALTER TABLE orders ADD COLUMN IF NOT EXISTS stop_price DECIMAL(40, 8)").await?;
    run_sql(pool, "ALTER TABLE orders ADD COLUMN IF NOT EXISTS reduce_only BOOLEAN NOT NULL DEFAULT false").await?;
    run_sql(pool, "ALTER TABLE orders ADD COLUMN IF NOT EXISTS time_in_force VARCHAR(10) NOT NULL DEFAULT 'GTC'").await?;
    run_sql(pool, "ALTER TABLE futures_positions ADD COLUMN IF NOT EXISTS take_profit DECIMAL(40, 8)").await?;
    run_sql(pool, "ALTER TABLE futures_positions ADD COLUMN IF NOT EXISTS stop_loss DECIMAL(40, 8)").await?;

    tracing::info!("Database migrations completed");
    Ok(())
}
