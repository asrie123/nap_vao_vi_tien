#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

// Định nghĩa cấu trúc lưu trữ dữ liệu (State)
#[contracttype]
pub enum DataKey {
    // Lưu trữ số dư của từng người dùng cụ thể
    Balance(Address),
}

#[contract]
pub struct WalletContract;

#[contractimpl]
impl WalletContract {
    /// Hàm nạp tiền (nhận token) vào Contract
    /// - `user`: Địa chỉ ví của người nạp
    /// - `token_address`: Địa chỉ của loại token (ví dụ: XLM)
    /// - `amount`: Số lượng tiền nạp
    pub fn deposit(env: Env, user: Address, token_address: Address, amount: i128) {
        // Yêu cầu người dùng (user) phải ký xác nhận giao dịch này
        user.require_auth();

        // Khởi tạo client để tương tác với token
        let token_client = token::Client::new(&env, &token_address);
        
        // Chuyển tiền từ ví người dùng vào địa chỉ của Smart Contract
        token_client.transfer(&user, &env.current_contract_address(), &amount);

        // Cập nhật số dư của người dùng trong bộ nhớ (storage) của contract
        let key = DataKey::Balance(user.clone());
        let mut balance: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        balance += amount;
        env.storage().persistent().set(&key, &balance);
    }

    /// Hàm rút tiền từ Contract về ví
    /// - `user`: Địa chỉ ví của người rút
    /// - `token_address`: Địa chỉ của loại token
    /// - `amount`: Số lượng tiền muốn rút
    pub fn withdraw(env: Env, user: Address, token_address: Address, amount: i128) {
        // Yêu cầu người dùng (user) phải ký xác nhận
        user.require_auth();

        // Kiểm tra số dư của người dùng trong contract có đủ để rút không
        let key = DataKey::Balance(user.clone());
        let mut balance: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        if balance < amount {
            panic!("Số dư không đủ để rút");
        }

        // Trừ đi số dư trong bộ nhớ của contract
        balance -= amount;
        env.storage().persistent().set(&key, &balance);

        // Chuyển tiền từ Contract trả lại về ví của người dùng
        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer(&env.current_contract_address(), &user, &amount);
    }

    /// Hàm xuất ra số dư hiện tại của một người dùng trong Contract
    pub fn get_user_balance(env: Env, user: Address) -> i128 {
        let key = DataKey::Balance(user);
        env.storage().persistent().get(&key).unwrap_or(0)
    }
    
    /// Hàm xuất ra TỔNG số tiền mà Contract đang giữ (của tất cả mọi người)
    pub fn get_contract_total_balance(env: Env, token_address: Address) -> i128 {
        let token_client = token::Client::new(&env, &token_address);
        token_client.balance(&env.current_contract_address())
    }
}
