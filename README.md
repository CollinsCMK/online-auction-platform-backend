# Digital Trading Platform - Online Auction System

A simplified online auction platform built with Actix Web and PostgreSQL, allowing users to bid on items and receive WhatsApp notifications.

## Features

- Create and manage auction listings with unique IDs, descriptions, base prices, and time windows
- Place bids equal to, higher than, or lower than base price during active trading windows  
- Track current best offers and winning bids
- View auction status (upcoming, active, ended)
- Basic admin interface for creating listings
- WhatsApp notifications when auctions end
- Group auctions by Auction Number
- View auction details including base price, best offer, volume etc.

## Tech Stack

- Backend: Rust + Actix Web framework
- Database: PostgreSQL
- WhatsApp Integration: WhatsApp Business API

## Prerequisites

- Rust 1.50+
- PostgreSQL 12+
- WhatsApp Business API access

## Setup

1. Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install sea-orm-cli:
```bash
cargo install sea-orm-cli@1.1.0
```

3. Make sure PostgreSQL is installed

4. Clone the repository:
```bash
git clone https://github.com/CollinsCMK/online-auction-platform-backend.git
cd online-auction-platform-backend
```

5. Set up the database:
```bash
psql -U postgres
CREATE DATABASE auction_system;
```

6. Set environment variables:
```bash
mv env.example .env
# Edit .env with your configuration
```

7. Run database migrations:
```bash
chmod +x sea-orm.sh
./sea-orm.sh
```

8. Configure WhatsApp Business API credentials in .env

9. Start the application:
```bash
cargo run
```

The server will start on http://localhost:8080

## API Documentation

### Auctions

- `GET /api/auctions` - List all auctions
- `GET /api/auctions/{id}` - Get auction details
- `POST /api/auctions` - Create new auction
- `POST /api/auctions/{id}/bids` - Place bid

### Bidding

- `GET /api/bids` - View bid history
- `GET /api/bids/{id}` - Get bid details

## Database Schema

```sql
-- Auctions
CREATE TABLE auctions (
  id SERIAL PRIMARY KEY,
  auction_number VARCHAR NOT NULL,
  title TEXT NOT NULL,
  base_price DECIMAL NOT NULL,
  start_time TIMESTAMP NOT NULL,
  end_time TIMESTAMP NOT NULL
);

-- Bids
CREATE TABLE bids (
  id SERIAL PRIMARY KEY,
  auction_id INTEGER REFERENCES auctions(id),
  amount DECIMAL NOT NULL,
  user_id INTEGER NOT NULL,
  created_at TIMESTAMP DEFAULT NOW()
);
```

## WhatsApp Integration

This project uses the official WhatsApp Business API for notifications. A valid Business API account and credentials are required.

Setup:
1. Register for WhatsApp Business API
2. Configure API credentials in .env
3. Implement webhook handlers for status updates

Limitations:
- Requires approved WhatsApp Business account
- Message template approval required
- API rate limits apply

## Deployment

The application will be deployed on Vultr:

1. Create Vultr instance
2. Install dependencies 
3. Set up PostgreSQL
4. Configure environment variables
5. Build and run application
6. Configure SSL/TLS
7. Set up monitoring

## Contributing

1. Fork the repo
2. Create feature branch
3. Commit changes
4. Push to branch
5. Create Pull Request

## License

MIT