use derive_more::From;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct Badges {
    appveyor: Option<Appveyor>,
    circle_ci: Option<CircleCi>,
    cirrus_ci: Option<CirrusCi>,
    gitlab: Option<Gitlab>,
    azure_devops: Option<AzureDevops>,
    travis_ci: Option<TravisCi>,
    bitbucket_pipelines: Option<BitbucketPipelines>,
    codecov: Option<Codecov>,
    coveralls: Option<Coveralls>,
    is_it_maintained_issue_resolution: Option<IsItMaintainedIssueResolution>,
    is_it_maintained_open_issues: Option<IsItMaintainedOpenIssues>,
    maintenance: Option<Maintenance>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, From, Serialize)]
#[serde(untagged)]
pub enum Badge {
    Appveyor(Appveyor),
    CircleCi(CircleCi),
    CirrusCi(CirrusCi),
    Gitlab(Gitlab),
    AzureDevops(AzureDevops),
    TravisCi(TravisCi),
    BitbucketPipelines(BitbucketPipelines),
    Codecov(Codecov),
    Coveralls(Coveralls),
    IsItMaintainedIssueResolution(IsItMaintainedIssueResolution),
    IsItMaintainedOpenIssues(IsItMaintainedOpenIssues),
    Maintenance(Maintenance),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Appveyor {
    repository: String,
    branch: String,
    service: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CircleCi {
    repository: String,
    branch: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CirrusCi {
    repository: String,
    branch: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Gitlab {
    repository: String,
    branch: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AzureDevops {
    project: String,
    pipeline: String,
    build: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TravisCi {
    repository: String,
    branch: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BitbucketPipelines {
    repository: String,
    branch: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Codecov {
    repository: String,
    branch: String,
    service: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Coveralls {
    repository: String,
    branch: String,
    service: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IsItMaintainedIssueResolution {
    repository: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IsItMaintainedOpenIssues {
    repository: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Maintenance {
    status: String,
}
