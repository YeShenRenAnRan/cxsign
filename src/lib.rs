// Copyright (C) 2025 worksoup <https://github.com/worksoup/>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub fn run() {
    use cxlib::{
        AccountCmdApp, AccountsCmdApp, AppTrait, CmdApp, CmdAppContext, CoursesCmdApp,
        LocationCmdApp, LocationsCmdApp, SignMainApp, WhereIsConfigCmdApp,
    };
    fn init(self_: &CmdApp<CmdAppContext>) -> (CmdAppContext, ()) {
        use cxlib::{
            captcha::CaptchaType,
            default_impl::store::{
                AccountTable, AliasTable, CourseTable, DataBase, ExcludeTable, LocationTable,
            },
            store::Dir,
        };
        if let Some(captcha_type) = std::env::var("CX_CAPTCHA_TYPE")
            .ok()
            .and_then(|s| s.parse().ok())
        {
            let _ = CaptchaType::set_global_default(&captcha_type);
        }
        let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
        let mut builder = env_logger::Builder::from_env(env);
        builder.target(env_logger::Target::Stderr);
        builder.init();
        Dir::set_config_dir_info(
            "TEST_CXSIGN",
            "up.workso",
            "Worksoup",
            env!("CARGO_PKG_NAME"),
        );
        let db = DataBase::default();
        db.add_table::<AccountTable>();
        db.add_table::<ExcludeTable>();
        db.add_table::<AliasTable>();
        db.add_table::<LocationTable>();
        db.add_table::<CourseTable>();
        (CmdAppContext::new(db, self_.command().clone()), ())
    }
    let cmd_app = CmdApp::new(clap::command!())
        .main_app::<SignMainApp>(Default::default())
        .meta_app(AccountCmdApp)
        .meta_app(AccountsCmdApp)
        .meta_app(CoursesCmdApp)
        .meta_app(LocationCmdApp)
        .meta_app(LocationsCmdApp)
        .meta_app(WhereIsConfigCmdApp);
    #[cfg(feature = "completion")]
    let cmd_app = cmd_app.meta_app(cxlib::CompletionCmdApp);
    cmd_app.init_and_run(init)
}
