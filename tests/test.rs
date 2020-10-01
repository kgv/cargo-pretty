use anyhow::Result;
use cargo_pretty::{Format, Settings};
use toml_lalrpop::TomlParser;

#[cfg(test)]
mod test {
    use super::*;

    mod badges {
        use super::*;

        const TARGET: &str = "\n\
            [badges]\n\
            appveyor = { repository = \"...\", branch = \"master\", service = \"github\" }\n\
            circle-ci = { repository = \"...\", branch = \"master\" }\n\
            cirrus-ci = { repository = \"...\", branch = \"master\" }\n\
            gitlab = { repository = \"...\", branch = \"master\" }\n\
            azure-devops = { project = \"...\", pipeline = \"...\", build = \"2\" }\n\
            travis-ci = { repository = \"...\", branch = \"master\" }\n\
            bitbucket-pipelines = { repository = \"...\", branch = \"master\" }\n\
            codecov = { repository = \"...\", branch = \"master\", service = \"github\" }\n\
            coveralls = { repository = \"...\", branch = \"master\", service = \"github\" }\n\
            is-it-maintained-issue-resolution = { repository = \"...\" }\n\
            is-it-maintained-open-issues = { repository = \"...\" }\n\
            maintenance = { status = \"...\" }\n\
        ";

        #[test]
        fn order() -> Result<()> {
            const SOURCE: &str = r#"
                [badges]
                appveyor = { service = "github", branch = "master", repository = "..." }
                azure-devops = { build = "2", pipeline = "...", project = "..." }
                bitbucket-pipelines = { branch = "master", repository = "..." }
                circle-ci = { branch = "master", repository = "..." }
                cirrus-ci = { branch = "master", repository = "..." }
                codecov = { service = "github", branch = "master", repository = "..." }
                coveralls = { service = "github", branch = "master", repository = "..." }
                gitlab = { branch = "master", repository = "..." }
                is-it-maintained-issue-resolution = { repository = "..." }
                is-it-maintained-open-issues = { repository = "..." }
                maintenance = { status = "..." }
                travis-ci = { branch = "master", repository = "..." }
            "#;
            let mut manifest = TomlParser::new().parse(SOURCE)?;
            let settings = Settings::default();
            let formated = format!("{}", manifest.format(&settings));
            assert_eq!(formated, TARGET);
            Ok(())
        }

        #[test]
        fn inline() -> Result<()> {
            const SOURCE: &str = r#"
                [badges.appveyor]
                repository = "..."
                branch = "master"
                service = "github"

                [badges.circle-ci]
                repository = "..."
                branch = "master"

                [badges.cirrus-ci]
                repository = "..."
                branch = "master"

                [badges.gitlab]
                repository = "..."
                branch = "master"

                [badges.azure-devops]
                project = "..."
                pipeline = "..."
                build = "2"

                [badges.travis-ci]
                repository = "..."
                branch = "master"

                [badges.bitbucket-pipelines]
                repository = "..."
                branch = "master"

                [badges.codecov]
                repository = "..."
                branch = "master"
                service = "github"

                [badges.coveralls]
                repository = "..."
                branch = "master"
                service = "github"

                [badges.is-it-maintained-issue-resolution]
                repository = "..."

                [badges.is-it-maintained-open-issues]
                repository = "..."

                [badges.maintenance]
                status = "..."
            "#;
            let mut manifest = TomlParser::new().parse(SOURCE)?;
            let settings = Settings::default();
            let formated = format!("{}", manifest.format(&settings));
            assert_eq!(formated, TARGET);
            Ok(())
        }
    }
}
