# SEU Utils

## 简介

顾名思义，这是一个用于东南大学学生的工具集合，目前包括以下功能：

- 获取选课系统token和batch_id
- 选课课程查询（推荐课程）
- 生成选课计划并自动选课
- 课程成绩查询
- 绩点计算

---

## 使用方法

**注意**

- 目前还未发布二进制文件，需要自行编译
- 强烈建议把各类文件放在可执行文件的 `resource` 路径下
- 除去特殊标记 **IN_NEED** 的文件，其他文件可以为无意义空文件

---

### 获取选课系统token和batch_id

```text
Usage: get_tokens.exe [OPTIONS]

Options:
      --config-yaml <CONFIG_YAML>  配置文件路径 IN_NEED [default: resource/config.yaml]
      --captcha-png <CAPTCHA_PNG>  临时验证码路径 [default: resource/captcha.png]
  -h, --help                       Print help
  -V, --version                    Print version
```

#### 配置文件解析

```yaml
token: 
batch_id: 
loginname: <your loginname>
password: <your password>
```

`token` 和 `batch_id` 留空即可，`password` 需要你自行提前打开选课系统，在开发者调试台中查看网络请求 `login` 的负载获取。

---

### 选课课程查询 & 课程信息解析 & 生成选课计划并自动选课

```text
Usage: choose_classes.exe [OPTIONS]

Options:
      --config-yaml <CONFIG_YAML>    配置文件路径 IN_NEED [default: resource/config.yaml]
      --classes-json <CLASSES_JSON>  全部课程路径 [default: resource/classes.json]
      --choose-json <CHOOSE_JSON>    选择课程路径 [default: resource/choose.json]
  -h, --help                         Print help
  -V, --version                      Print version
```

配置文件同上，`classes.json` 为全部课程信息，`choose.json` 为你想要选择的课程信息。

### 课程成绩查询

```text
Usage: get_grades.exe [OPTIONS]

Options:
  -c, --cookie-txt <COOKIE_TXT>            cookie路径 IN_NEED [default: resource/grades_cookie.txt]
  -g, --grades-json <GRADES_JSON>          保存导出成绩的json文件路径 [default: resource/grades.json]
  -r, --raw-grades-json <RAW_GRADES_JSON>  保存原始成绩的json文件路径 [default: resource/raw_grades.json]
  -h, --help                               Print help
  -V, --version                            Print version
```

cookie 需要自行打开选课系统，在开发者调试台中查看 `*.do` 的请求头获取。此cookie时效性很强，建议每次使用前都重新获取。

### 绩点计算

```text
Usage: calc_gpa.exe [OPTIONS]

Options:
      --grades-json <GRADES_JSON>  保存导出成绩的json文件路径 IN_NEED [default: resource/grades.json]
      --terms <TERMS>              学期列表，逗号隔开，不填则默认计算所有学期 [example: 2024-2025-1, 2024-2025-2] [default: ]
  -h, --help                       Print help
  -V, --version                    Print version
```

学期列表的格式为 `yyyy-yyyy-x`，其中 `x` 为学期，`1` 为暑期学校，`2` 为秋季学期，`3` 为春季学期。