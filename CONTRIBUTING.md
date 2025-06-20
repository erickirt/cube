# Contributing to Cube

Thanks for taking the time for contribution to Cube!
We're very welcoming community and while it's very much appreciated if you follow these guidelines it's not a requirement.

## Code of Conduct
This project and everyone participating in it is governed by the [Cube Code of Conduct](./CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code. Please report unacceptable behavior to conduct@cube.dev.

## Contributing Code Changes

Please review the following sections before proposing code changes. 

### License

- Cube Client is [MIT licensed](./packages/cubejs-client-core/LICENSE).
- Cube Backend is [Apache 2.0 licensed](./packages/cubejs-server/LICENSE).

### Developer Certificate of Origin (DCO)

By contributing to Cube Dev, Inc., You accept and agree to the terms and conditions in the [Developer Certificate of Origin](https://github.com/cube-js/cube/blob/master/DCO.md) for Your present and future Contributions submitted to Cube Dev, Inc. Your contribution includes any submissions to the [Cube repository](https://github.com/cube-js) when you click on such buttons as `Propose changes` or `Create pull request`. Except for the licenses granted herein, You reserve all right, title, and interest in and to Your Contributions.

## Step-by-step guide to contributing

1. Find [issues](https://github.com/cube-js/cube/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc) where we need help. Search for issues with either [`good first issue`](https://github.com/cube-js/cube/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+label%3A%22good+first+issue%22+) and/or [`help wanted`](https://github.com/cube-js/cube/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+label%3A%22help+wanted%22) labels.
2. Follow the directions in the [Getting Started guide](https://cube.dev/docs/getting-started) to get Cube up and running (incl. the [Developer Playground](https://cube.dev/docs/dev-tools/dev-playground)). 
3. Clone the [Cube repo](https://github.com/cube-js/cube).
4. Submit your Pull Request. 
5. Testing: Please include test(s) for your code contribution. Depending on a change it can be tested by unit, integration or E2E test. See some of the test examples for [drivers](https://github.com/cube-js/cube/pull/1333/commits/56dadccd62ac4eaceafe650d2853406f5d3d9d43) and [backend](https://github.com/cube-js/cube/tree/master/packages/cubejs-backend-shared/test). There're separate packages for [E2E testing](https://github.com/cube-js/cube/tree/master/packages/cubejs-testing/) and [E2E driver testing](https://github.com/cube-js/cube/tree/master/packages/cubejs-testing-drivers/). **Tests are required for most of the contributions.**
6. Documentation: When new features are added or there are changes to existing features that require updates to documentation, we encourage you to add/update any missing documentation in the [`/docs` folder](https://github.com/cube-js/cube/tree/master/docs). To update an existing documentation page, you can simply click on the `Edit this page` button on the top right corner of the documentation page. 
7. Relevant team(s) will be pinged automatically for a review based on information in the `CODEOWNERS` file. 

## Development Workflow

### Prerequisites

Cube works with Node.js 20+ and uses Yarn as a package manager.

### Cube Docker

Cube offers two different types of Docker image:

- Stable (building from published release on npm)
- Dev (building from source files, needed to test unpublished changes)

For more information, take a look at [Docker Development Guide](./packages/cubejs-docker/DEVELOPMENT.md).

#### Stable Docker Release

1. After cloning Cube repository run `yarn install` in `packages/cubejs-docker` to install dependencies.
2. Copy `yarn.lock` file from the project root to the `packages/cubejs-docker` folder and use `docker build -t cubejs/cube:latest -f latest.Dockerfile` in `packages/cubejs-docker` to build stable docker image manually.

#### Development

1. After cloning Cube repository run `yarn install` to install dependencies.
2. Use `docker build -t cubejs/cube:dev -f dev.Dockerfile ../../` in `packages/cubejs-docker` to build stable development image.

### Cube Client

1. After cloning Cube repository run `yarn install` in root directory.
2. Use `yarn link` to add these packages to link registry.
3. Perform required code changes.
4. Use `yarn build` in the repository root to build CommonJS and UMD modules.
5. Use `yarn link @cubejs-client/core` and/or `yarn link @cubejs-client/react` in your project to test changes applied.
6. Use `yarn test` where available to test your changes.
7. Ensure that any CommonJS and UMD modules are included as part of your commit.

To get set up quickly, you can perform 1) and 2) with one line from the `cube` clone root folder:

```
$ cd packages/cubejs-client-core && yarn && yarn link && cd ../.. && cd packages/cubejs-client-react && yarn && yarn link && cd ../..
```

### Cube Server

#### Prerequisites

If you are going to develop a JDBC driver, you need to [install Java with JDK][link-java-guide].

[link-java-guide]:
https://github.com/cube-js/cube/blob/master/packages/cubejs-jdbc-driver/README.md#java-installation

#### Development

Cube.is written in a mixture of JavaScript, TypeScript, and Rust. TypeScript and Rust are preferred for new code.

> Attention: Cube uses TypeScript configured in incremental mode, which uses cache to speed up compilation,  
> but in some cases, you can run into a problem with a not recompiled file. To fix it, we recommend running `$ yarn clean` and `$ yarn tsc`.

1. Clone the Cube repository, `git clone https://github.com/cube-js/cube`. 
2. Run `yarn install` in the root directory.
3. Run `yarn build` in the root directory to build the frontend dependent packages. 
4. Run `yarn build` in `packages/cubejs-playground` to build the frontend.
5. Run `yarn tsc:watch` to start the TypeScript compiler in watch mode.
6. Run `yarn link` in `packages/cubejs-<pkg>` for the drivers and dependent packages you intend to modify. 
7. Run `yarn install` in `packages/cubejs-<pkg>` to install dependencies for drivers and dependent packages.
8. Run `yarn link @cubejs-backend/<pkg>` in `packages/cubejs-server-core` to link drivers and dependent packages.
9. Run `yarn link` in `packages/cubejs-server-core`.
10. Create or choose an existing project for testing. You can generate a new one with 
    [cubejs-cli](https://cube.dev/docs/reference/cli) tool.
11. Run `yarn link @cubejs-backend/server-core` in your project directory. 
12. Run `yarn dev` to start your testing project and verify changes.

Instead of running all of the above commands manually you can use the `dev_env_setup.sh` script:

1. Clone the Cube repository, `git clone https://github.com/cube-js/cube`.
2. Navigate to your working projects directory and run `/path/to/cube/repo/dev_env_setup.sh`. The script will
   ask you some questions and run all the required commands. In case you decide to create a new testing project,
   it will be created in the current directory (that is why you probably don't want to run this script within 
   cube repo directory).

### Debugging with WebStorm

1. Follow all the steps from the previous section. Make sure that the `yarn tsc:watch` daemon is running in the background.
2. Open the Cube project in WebStorm.
3. Create a new configuration, using `./node_modules/.bin/cubejs-server` for Node Parameters and the directory of your test project for Working directory.
4. Run/Debug dev Cube servers using the new configuration.

## Contributing Database Drivers

To enhance the adoption of community-contributed drivers, we decided to split the database driver contribution process into multiple stages.

1. Each driver which is planned to be contributed to the main Cube repository should be published first as an npm package. Please see [Publishing Driver npm package](#publishing-driver-npm-package) on how to do that.
2. This NPM package should be contributed to the list of [Third-party community drivers](https://cube.dev/docs/config/databases#third-party-community-drivers).
3. Please make sure each npm package has a README with instructions on how to install it to the official docker image and how to connect it to the database.
4. Posting a backlink to an open-source repository would be a good idea here so people can provide feedback on it by posting issues.
5. Before creating PR for the main repository, please make sure it's tested with the standard Cube E2E testing suite. An example of an E2E testing suite can be found here: https://github.com/cube-js/cube/blob/master/packages/cubejs-testing/test/driver-postgres.test.ts
6. If you're creating PR for the main repo, please be prepared to become a maintainer for this driver and dedicate some time to it. There're no specific time requirements. As a rule of thumb, you should expect to spend time on a weekly basis.
7. Due to limited resources Core team will review and merge driver PRs based on popularity and development activity. Preference is given to drivers that are used by a significant number of users.

### Implementing a Driver

1. Copy existing driver package structure and name it in `@cubejs-backend/<db-name>-driver` format.
`@cubejs-backend/mysql-driver` is a very good candidate for copying this structure.
2. Please do not copy *CHANGELOG.md*.
3. Name driver class and adjust package.json, README.md accordingly.
4. As a rule of thumb please use only pure JS libraries as a dependencies where possible.
It increases driver adoption rate a lot.
5. Typically, you need to implement only `query()` and `testConnection()` methods of driver.
The rest will be done by `BaseDriver` class.
6. If db requires connection pooling prefer use `generic-pool` implementation with settings similar to other db packages.
7. Make sure your driver has `release()` method in case DB expects graceful shutdowns for connections.
8. Please use yarn to add any dependencies and run `$ yarn` within the package before committing to ensure right `yarn.lock` is in place.
9. Add this driver dependency to [cubejs-server-core/core/DriverDependencies.js](https://github.com/cube-js/cube/blob/master/packages/cubejs-server-core/core/DriverDependencies.js#L1).

### Implementing a JDBC Driver

It is recommended to implement native, non-JDBC drivers for databases. Even though implementing
a JDBC driver might seem like a quick solution, its reliance on external libraries makes it
harder to maintain. Also, such drivers often lack support for important features, such as export buckets and various authentication methods.

### Implementing SQL Dialect

1. Find the most similar `BaseQuery` implementation in `@cubejs-backend/schema-compiler/adapter`.
2. Copy it, adjust SQL generation accordingly and put it in driver package. Driver package will obtain `@cubejs-backend/schema-compiler` dependency from that point.
3. Add `static dialectClass()` method to your driver class which returns `BaseQuery` implementation for the database. For example:
```javascript
const { BaseDriver } = require('@cubejs-backend/query-orchestrator');
const FooQuery = require('./FooQuery');

class FooDriver extends BaseDriver {
  // ...
  static dialectClass() {
    return FooQuery;
  }
}
```
If driver class contains `static dialectClass()` method it'll be used to lookup corresponding SQL dialect. Otherwise, it will use the default dialect for the database type.

### Publishing Driver npm Package

Cube looks up `cubejs-{dbType}-driver` package among installed modules to fullfil driver dependency if there's no corresponding default driver for the specified database type.
For example one can publish `cubejs-foo-driver` npm package to fullfil driver dependency for the `foo` database type.

## Other Packages

### Testing Schema Compiler

In order to run tests in `cubejs-schema-compiler` package you need to have running [Docker](https://docs.docker.com/install/) on your machine.
When it's up and running just use `yarn test` in `packages/cubejs-schema-compiler` to execute tests.

### Client Packages

If you want to make changes to the Cube.js client packages and test them locally in your project you can do it the following way:
1. Make the desired changes and run `yarn build` in the root directory (you can also use `yarn watch`)
2. Go to the `~/some-path/cube.js/packages/cubejs-client-core` directory and run `yarn link`. (You'll see the messages _Registered **"@cubejs-client/core"**_)
3. Now you can link it in your project (e.g. _/my-project/dashboard-app_). You can do so running `yarn link "@cubejs-client/core"`

If you want to make changes to the `@cubejs-client/react` package you'll need a few extra steps
1. Go to your project's **node_modules** directory and find the react package (e.g. _/my-project/dashboard-app/node_modules/react_ and run `yarn link`
2. Go to the `~/some-path/cube.js/packages/cubejs-client-react` directory and run `yarn link react`

Now your project will be using the local packages.

**NOTE:** You might need to restart your project after linking the packages.

### Rust Packages

Please use `cargo test` to test packages and `cargo fmt` to format code before commit.

## Style guides

We're passionate about what code can do rather how it's formatted.
But in order to make code and docs maintainable following style guides will be enforced.
Following these guidelines is not a requirement, but you can save some time for maintainers if you apply those to your contribution beforehand.

### Code

1. Run `yarn lint` in package before committing your changes.
If package doesn't have lint script, please add it and run.
There's one root `.eslintrc.js` file for all packages except client ones.
Client packages has it's own `.eslintrc.js` files.
2. Run `yarn test` before committing if package has tests.
3. Please use [conventional commits name](https://www.conventionalcommits.org/) for your PR.
It'll be used to build change logs.
All PRs are merged using the squash strategy. PR title usually would be used as a name for commit. So please make sure it has a sensible name.  
4. For the scope part of commit name please use package name if it's within one package or don't use it if change spans multiple packages. For example `feat(server-core):` or `fix(cubestore):`.
5. Commit messages that are getting merged should contain mostly "Why" those changes are made as opposed to "What" changes are done. "Why" can be a feature, reference to issue or reasons to fix something like a chore.
6. Do not reformat code you aren't really changing unless it's absolutely necessary (e.g. fixing linter). Such changes make it really hard to use git blame feature when we need to find a commit where line change of interest was introduced. Please do not include files that contain only reformatting changes in the commit.
