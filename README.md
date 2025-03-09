# SEU Utils 🛠️

## 简介 🚀

SEU Utils 是一款专为东南大学（SEU）学生打造的工具集合，旨在提升选课、成绩查询以及绩点计算等日常操作的效率。当前包含以下功能：

- 获取选课系统token和batch_id
- 选课课程查询（推荐课程）
- 生成选课计划并自动选课
- 课程成绩查询
- 绩点计算

**新增：**[桌面应用版本](https://github.com/twilight0702/SEU-utils-frontend)，提供可视化操作页面

---

## 使用指南 📚

**⚠️ 注意事项：**

- 作者能力精力财力有限，本工具仅保证 64bit Windows
  平台的正常使用，其他平台请自行测试，如果你愿意对多平台进行适配，请提交 [PR](https://github.com/harkerhand/seu_utils/pulls)
- 本工具仅供学习交流使用，不得用于任何商业用途，否则后果自负
- 所有的可执行文件都需要在命令行中运行，不支持图形化界面
- 特殊标记 **IN_NEED** 的文件，需要手动提供，其他文件会自动生成
- 默认的资源文件路径为 `resource/`，可以通过命令行参数进行修改，但不建议修改

---

### 获取选课系统token和batch_id

**编译命令**

```text
cargo run -p choose_classes --bin get_tokens -- <OPTIONS>
```

**使用说明**

```text

Usage: get_tokens.exe [OPTIONS]

Options:
      --config-yaml <CONFIG_YAML>  配置文件路径 IN_NEED [default: resource/config.yaml]
      --captcha-png <CAPTCHA_PNG>  临时验证码路径 [default: resource/captcha.png]
  -h, --help                       Print help
  -V, --version                    Print version
```

#### 配置文件解析 `config.yaml`

```yaml
token: 
batch_id: 
loginname: <your loginname>
password: <your password>
```

- `token` 留空即可
- `batch_id` 需要你进入选课系统后，在开发者调试台中查看网络请求 `list` 的请求头 `Referer` 字段获取
- `loginname` 为你的一卡通号
- `password` 需要你在选课系统登录页面，点击登录后，在开发者调试台中查看网络请求 `login` 的负载 `password` 字段获取。

---

### 选课课程查询 & 课程信息解析 & 生成选课计划并自动选课

**编译命令**

```text
cargo run -p choose_classes --bin choose_classes -- <OPTIONS>
```

**使用说明**

```text
Usage: choose_classes.exe [OPTIONS]

Options:
      --config-yaml <CONFIG_YAML>    配置文件路径 IN_NEED [default: resource/config.yaml]
      --classes-json <CLASSES_JSON>  全部课程路径 [default: resource/classes.json]
      --choose-json <CHOOSE_JSON>    选择课程路径 [default: resource/choose.json]
  -h, --help                         Print help
  -V, --version                      Print version
```

- 配置文件同上
- `classes.json` 为全部课程信息，`choose.json` 为你想要选择的课程信息，会自动生成

---

### 课程成绩查询

**编译命令**

```text
cargo run -p get_grades --bin get_grades -- <OPTIONS>
```

**使用说明**

```text
Usage: get_grades.exe [OPTIONS]

Options:
  -c, --cookie-txt <COOKIE_TXT>            cookie路径 IN_NEED [default: resource/grades_cookie.txt]
  -g, --grades-json <GRADES_JSON>          保存导出成绩的json文件路径 [default: resource/grades.json]
  -r, --raw-grades-json <RAW_GRADES_JSON>  保存原始成绩的json文件路径 [default: resource/raw_grades.json]
  -h, --help                               Print help
  -V, --version                            Print version
```

cookie 需要进入成绩查询系统后，在开发者调试台中查看任意后缀为 `.do` 的请求头 `Cookie` 字段获取。此cookie时效性很强，建议每次使用前都重新获取。

---

### 绩点计算

**编译命令**

```text
cargo run -p get_grades --bin calc_gpa -- <OPTIONS>
```

**使用说明**

```text
Usage: calc_gpa.exe [OPTIONS]

Options:
      --grades-json <GRADES_JSON>  保存导出成绩的json文件路径 IN_NEED [default: resource/grades.json]
      --terms <TERMS>              学期列表，逗号隔开，不填则默认计算所有学期 [example: 2024-2025-1, 2024-2025-2] [default: ]
  -h, --help                       Print help
  -V, --version                    Print version
```

学期列表的格式为 `yyyy-yyyy-x`，其中 `x` 为学期，`1` 为暑期学校，`2` 为秋季学期，`3` 为春季学期。
