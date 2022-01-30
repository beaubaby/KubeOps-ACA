# GIT Introduction

using [GITHUB](https://www.github.com/)

Subversion Control System (Traditional world)
VCS = Version Control System

Semantic version (Semver) = Standard version for Cloud Native

Reference:
[Semver Version Standard](https://semver.org/#semantic-versioning-specification-semver)

```
Example:
1.1.2
Major.Minor.Patch(Fix bug) - beta/alpha 
```

### Benefits for Semver

* Major using for add core, change core and breaking change

* Minor using for add feature, fix feature, implement feature

* Patch using for fix bug, typo

* Alpha using for release core feature

* beta using for ready to PROD but have known issue

### Commit code

Using [Commit Conventional](https://www.conventionalcommits.org/en/v1.0.0/)

### Asymatic key
Public key = Mother key -> Keep on the server
Private key = Brother key -> Keep on local host

`ssh-keygen -t <algorithm> -C "<message>"`

kubeOps uses `ed_25519` for key algorithm.

### Using git ssh

```
git clone git@kakkai:kakkai-academy/security-pipeline.git
```

#### Appendix

`git@host:organization/repository-in-org`

host refers from `~/.ssh/config`

```
Host host
  HostName github.com
  User <username>
  AddKeysToAgent yes
  UseKeychain yes
  IdentityFile ~/.ssh/<key>
```

###  Git switch between organization

![git_switch_org](/pic/git_switch_org.png)

Reference from JoJo:
https://docs.google.com/document/d/16fM2zhFwS4dHUYQtNqy3LGKc5MYxtQ_d6HqDiSvxhfw/edit