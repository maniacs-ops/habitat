// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

use std::net;

use num_cpus;

/// URL to GitHub API endpoint
pub const DEFAULT_GITHUB_URL: &'static str = "https://api.github.com";
/// Default Client ID for providing a default value in development environments only. This is
/// associated to the habitat-sh GitHub account and is configured to re-direct and point to a local
/// builder-api.
///
/// See https://github.com/settings/connections/applications/0c2f738a7d0bd300de10
pub const DEV_GITHUB_CLIENT_ID: &'static str = "0c2f738a7d0bd300de10";
/// Default Client Secret for development purposes only. See the `DEV_GITHUB_CLIENT_ID` for
/// additional comments.
pub const DEV_GITHUB_CLIENT_SECRET: &'static str = "438223113eeb6e7edf2d2f91a232b72de72b9bdf";

pub trait DispatcherCfg {
    fn default_worker_count() -> usize {
        // JW TODO: increase default count after r2d2 connection pools are moved to be owned
        // by main thread of servers instead of dispatcher threads.
        // num_cpus::get() * 8
        num_cpus::get()
    }

    fn worker_count(&self) -> usize;
}

pub trait GitHubOAuth {
    fn github_url(&self) -> &str;
    fn github_client_id(&self) -> &str;
    fn github_client_secret(&self) -> &str;
}

pub trait RouteAddrs {
    fn route_addrs(&self) -> &Vec<net::SocketAddrV4>;

    fn heartbeat_port(&self) -> u16 {
        5563
    }
}

pub trait Shards {
    fn shards(&self) -> &Vec<u32>;
}

pub trait ToAddrString {
    fn to_addr_string(&self) -> String;
}

impl ToAddrString for net::SocketAddrV4 {
    fn to_addr_string(&self) -> String {
        format!("tcp://{}:{}", self.ip(), self.port())
    }
}
