// Copyright 2020 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Key-Value store backed with a local sled::Tree.
//!
//! `KV` talks the same API defined in `KVApi`.
//!
//! `KV` behave exactly the same as a metasrv without distributed logs(raft), since it is driven by
//! a embedded raft `StateMachine`.

mod kv;

#[cfg(test)]
mod kv_test;

pub use kv::KV;
