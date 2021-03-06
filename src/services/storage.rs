//! This module contains the implementation of a service to
//! use local and session storage of a browser.

use stdweb::Value;
use format::{Storable, Restorable};

/// Represents errors of a storage.
#[derive(Debug, Fail)]
enum StorageError {
    #[fail(display = "restore error")]
    CantRestore,
}

/// An area to keep the data in.
pub enum Area {
    /// Use `localStorage` of a browser.
    Local,
    /// Use `sessionStorage` of a browser.
    Session,
}

/// A storage service attached to a context.
pub struct StorageService {
    scope: Area,
}

impl StorageService {

    /// Creates a new storage service instance with specified storate scope.
    pub fn new(scope: Area) -> Self {
        StorageService { scope }
    }

    /// Stores value to the storage.
    pub fn store<T>(&mut self, key: &str, value: T)
    where
        T: Into<Storable>
    {
        if let Some(data) = value.into() {
            match self.scope {
                Area::Local => { js! { @(no_return)
                    localStorage.setItem(@{key}, @{data});
                } },
                Area::Session => { js! { @(no_return)
                    sessionStorage.setItem(@{key}, @{data});
                } },
            }
        }
    }

    /// Restores value from the storage.
    pub fn restore<T>(&mut self, key: &str) -> T
    where
        T : From<Restorable>
    {
        let value: Value = {
            match self.scope {
                Area::Local => js! { return localStorage.getItem(@{key}); },
                Area::Session => js! { return sessionStorage.getItem(@{key}); },
            }
        };
        let data = value.into_string().ok_or_else(|| StorageError::CantRestore.into());
        T::from(data)
    }

    /// Removes value from the storage.
    pub fn remove(&mut self, key: &str) {
        {
            match self.scope {
                Area::Local => js! { @(no_return)
                    localStorage.removeItem(@{key});
                },
                Area::Session => js! { @(no_return)
                    sessionStorage.removeItem(@{key});
                },
            }
        };
    }
}
