#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, Address, Env, String, Symbol,
};

/// ---------------------------
/// Errores (repr u32)
/// ---------------------------
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NombreVacio = 1,
    NombreMuyLargo = 2,
    NoAutorizado = 3,
    NoInicializado = 4,
}

/// ---------------------------
/// Claves de storage
/// ---------------------------
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    ContadorSaludos,
    UltimoSaludo(Address),
}

/// ---------------------------
/// Contrato
/// ---------------------------
#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    /// initialize(admin)
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::NoInicializado);
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32);

        env.storage().instance().extend_ttl(100, 100);
        Ok(())
    }

    /// hello(usuario, nombre:String) -> "Hola"
    /// 🔪 CORREGIDO: `String` para validar longitud
    pub fn hello(env: Env, usuario: Address, nombre: String) -> Result<Symbol, Error> {
        if !env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::NoInicializado);
        }

        if nombre.len() == 0 {
            return Err(Error::NombreVacio);
        }
        if nombre.len() > 32 {
            return Err(Error::NombreMuyLargo);
        }

        // Lee-modifica-guarda contador (instance)
        let key_contador = DataKey::ContadorSaludos;
        let contador: u32 = env.storage().instance().get(&key_contador).unwrap_or(0);
        env.storage()
            .instance()
            .set(&key_contador, &(contador.saturating_add(1)));

        // Guarda último saludo por usuario (persistent)
        env.storage()
            .persistent()
            .set(&DataKey::UltimoSaludo(usuario.clone()), &nombre);

        // TTLs
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::UltimoSaludo(usuario), 100, 100);
        env.storage().instance().extend_ttl(100, 100);

        Ok(Symbol::new(&env, "Hola"))
    }

    /// get_contador() -> u32
    pub fn get_contador(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ContadorSaludos)
            .unwrap_or(0)
    }

    /// get_ultimo_saludo(usuario) -> Option<String>
    pub fn get_ultimo_saludo(env: Env, usuario: Address) -> Option<String> {
        env.storage()
            .persistent()
            .get(&DataKey::UltimoSaludo(usuario))
    }

    /// reset_contador(caller) -> Result<(), Error>
    pub fn reset_contador(env: Env, caller: Address) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NoInicializado)?;

        if caller != admin {
            return Err(Error::NoAutorizado);
        }

        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32);
        Ok(())
    }
}

/// ---------------------------
/// Tests (usan String::from_str según la corrección)
/// ---------------------------
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);
        assert_eq!(client.get_contador(), 0);
    }

    #[test]
    #[should_panic(expected = "NoInicializado")]
    fn test_no_reinicializar() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);
        client.initialize(&admin); // segunda vez -> panic
    }

    #[test]
    fn test_hello_exitoso() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        client.initialize(&admin);

        let nombre = String::from_str(&env, "Ana"); // 🔪 CORREGIDO
        let res = client.hello(&usuario, &nombre);

        assert_eq!(res, Symbol::new(&env, "Hola"));
        assert_eq!(client.get_contador(), 1);
        assert_eq!(client.get_ultimo_saludo(&usuario), Some(nombre));
    }

    #[test]
    #[should_panic(expected = "NombreVacio")]
    fn test_nombre_vacio() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        client.initialize(&admin);

        let vacio = String::from_str(&env, "");
        client.hello(&usuario, &vacio); // -> panic
    }

    #[test]
    fn test_reset_solo_admin() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        client.initialize(&admin);

        client.hello(&usuario, &String::from_str(&env, "Test"));
        assert_eq!(client.get_contador(), 1);

        client.reset_contador(&admin);
        assert_eq!(client.get_contador(), 0);
    }

    #[test]
    #[should_panic(expected = "NoAutorizado")]
    fn test_reset_no_autorizado() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let otro = Address::generate(&env);
        client.initialize(&admin);

        client.reset_contador(&otro); // -> panic
    }
}
