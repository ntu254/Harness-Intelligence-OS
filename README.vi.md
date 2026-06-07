<div align="center">

# Harness Intelligence OS

### Lớp vận hành repository dành cho AI coding agents

**HI-OS giúp con người và AI coding agents đi từ ý tưởng đến công việc đã được kiểm chứng, thay vì để agent nhảy thẳng vào sửa code.**

<br />

<p>
  <a href="./README.md">English</a>
  ·
  <a href="./README.vi.md"><strong>Tiếng Việt</strong></a>
</p>

<br />

<p>
  <strong>intake</strong>
  &nbsp;→&nbsp;
  <strong>context pack</strong>
  &nbsp;→&nbsp;
  <strong>validation proof</strong>
  &nbsp;→&nbsp;
  <strong>trace</strong>
  &nbsp;→&nbsp;
  <strong>governance dashboard</strong>
</p>

<br />

<table>
  <tr>
    <td><strong>Sản phẩm</strong></td>
    <td>Harness Intelligence OS</td>
  </tr>
  <tr>
    <td><strong>Tên ngắn</strong></td>
    <td>HI-OS</td>
  </tr>
  <tr>
    <td><strong>Canonical origin</strong></td>
    <td><code>ntu254/Harness-Intelligence-OS</code></td>
  </tr>
  <tr>
    <td><strong>Dòng hiện tại</strong></td>
    <td><code>v0.7 Adoption Ready</code></td>
  </tr>
</table>

<br />

<strong>Cài được. Dễ hiểu. Dễ debug. Production-clean. Có thể kiểm chứng công khai.</strong>

</div>

---

## Mục Lục

- [HI-OS Là Gì](#hi-os-là-gì)
- [HI-OS Mang Lại Gì](#hi-os-mang-lại-gì)
- [Định Danh Chủ Quyền](#định-danh-chủ-quyền)
- [Quickstart 5 Phút](#quickstart-5-phút)
- [Chạy Lần Đầu](#chạy-lần-đầu)
- [Tạo Story Được Kiểm Chứng Đầu Tiên](#tạo-story-được-kiểm-chứng-đầu-tiên)
- [Workflow Cốt Lõi](#workflow-cốt-lõi)
- [Governance Dashboard](#governance-dashboard)
- [Trusted Distribution](#trusted-distribution)
- [Production-Clean Payload](#production-clean-payload)
- [CodeGraph Và NotebookLM Evidence](#codegraph-và-notebooklm-evidence)
- [Sơ Đồ Repository](#sơ-đồ-repository)
- [Nên Bắt Đầu Từ Đâu](#nên-bắt-đầu-từ-đâu)
- [Milestone Hiện Tại](#milestone-hiện-tại)
- [Đóng Góp](#đóng-góp)

---

## HI-OS Là Gì

HI-OS là một **lớp vận hành repository dành cho AI coding agents**.

Ứng dụng là thứ người dùng chạm vào.  
**HI-OS là thứ agent chạm vào.**

Phần lớn repository hiện nay để agent đọc file rời rạc, tự đoán ý định, rồi sửa code quá sớm. HI-OS thay đổi điều đó bằng cách ép repository đi qua một workflow có kiểm soát:

```text
human request
  → intake classification
  → story packet
  → context pack
  → implementation
  → validation proof
  → trace
  → story governance gate
  → governance dashboard
```

Thay vì bàn giao tự do, HI-OS bắt repository trả lời các câu hỏi thực tế trước:

```text
Công việc được yêu cầu là gì?
Luật sản phẩm hoặc kiến trúc nào áp dụng?
Thay đổi này rủi ro đến mức nào?
Agent cần đọc context nào?
Proof nào chứng minh công việc đã xong?
Release hoặc dashboard evidence nào để con người audit sau này?
```

---

## HI-OS Mang Lại Gì

<table>
  <tr>
    <th>Khả năng</th>
    <th>Mục đích</th>
  </tr>
  <tr>
    <td><strong>Intake</strong></td>
    <td>Phân loại công việc trước khi implementation.</td>
  </tr>
  <tr>
    <td><strong>Risk lanes</strong></td>
    <td>Tách Tiny, Normal và High-Risk changes.</td>
  </tr>
  <tr>
    <td><strong>Story packets</strong></td>
    <td>Biến mỗi work item thành một contract bền vững.</td>
  </tr>
  <tr>
    <td><strong>Context packs</strong></td>
    <td>Cho agent đúng context cần đọc, không phải đọc lan man cả repo.</td>
  </tr>
  <tr>
    <td><strong>Architecture checks</strong></td>
    <td>Chặn vi phạm boundary trước khi bàn giao.</td>
  </tr>
  <tr>
    <td><strong>Story verify</strong></td>
    <td>Không cho story hoàn tất nếu thiếu proof bắt buộc.</td>
  </tr>
  <tr>
    <td><strong>Release verify</strong></td>
    <td>Kiểm chứng binaries, SHA256 assets, payload ZIP, version và smoke command.</td>
  </tr>
  <tr>
    <td><strong>Governance dashboard</strong></td>
    <td>Xuất proof cục bộ thành static HTML dashboard.</td>
  </tr>
  <tr>
    <td><strong>CodeGraph / NotebookLM evidence</strong></td>
    <td>Dùng provider ngoài qua file-based boundary, không cho ghi thẳng vào SQLite.</td>
  </tr>
  <tr>
    <td><strong>Friction learning</strong></td>
    <td>Ghi nhận blocker và biến friction lặp lại thành tín hiệu backlog.</td>
  </tr>
</table>

---

## Định Danh Chủ Quyền

HI-OS có một product identity được tracking rõ ràng.

```text
Harness Intelligence OS
tên ngắn: HI-OS
repository: ntu254/Harness-Intelligence-OS
default release origin: ntu254/Harness-Intelligence-OS
```

Identity này nằm trong:

```text
hios.toml
```

Kiểm tra bằng:

```bash
scripts/bin/harness-cli identity
```

Trên Windows:

```powershell
.\scripts\bin\harness-cli.exe identity
```

Governance report, dashboard và release verification dùng identity này để đảm bảo release trust luôn khớp với canonical origin.

---

## Quickstart 5 Phút

Dùng phần này khi muốn cài HI-OS vào một project khác.

<details open>
<summary><strong>macOS / Linux</strong></summary>

```bash
curl -fsSL "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --yes
```

Merge mode, dùng khi project đã có sẵn `AGENTS.md`, `docs/` hoặc `scripts/`:

```bash
curl -fsSL "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --merge --yes
```

Cho Claude Code users:

```bash
curl -fsSL "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.sh?$(date +%s)" | bash -s -- --claude --yes
```

</details>

<details>
<summary><strong>Windows PowerShell</strong></summary>

```powershell
& ([scriptblock]::Create((irm "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.ps1"))) -Yes
```

Merge mode:

```powershell
& ([scriptblock]::Create((irm "https://raw.githubusercontent.com/ntu254/Harness-Intelligence-OS/main/scripts/install-harness.ps1"))) -Merge -Yes
```

</details>

Installer sẽ copy operating docs của Harness và tải trusted prebuilt CLI vào:

```text
scripts/bin/harness-cli
scripts/bin/harness-cli.exe
```

---

## Chạy Lần Đầu

Khởi tạo local Harness database:

```bash
scripts/bin/harness-cli init
scripts/bin/harness-cli query matrix
```

Trên Windows:

```powershell
.\scripts\bin\harness-cli.exe init
.\scripts\bin\harness-cli.exe query matrix
```

Local runtime evidence được ignore và không nên commit:

```text
harness.db
.harness/
```

---

## Tạo Story Được Kiểm Chứng Đầu Tiên

Tạo intake và story metadata:

```bash
scripts/bin/harness-cli intake \
  --type "change request" \
  --summary "Describe the requested change" \
  --lane normal \
  --story US-001 \
  --docs "docs/product/overview.md"

scripts/bin/harness-cli story add \
  --id US-001 \
  --title "First verified change" \
  --lane normal \
  --contract docs/product/overview.md \
  --verify "your-test-command"
```

Trên Windows, dùng cùng arguments với:

```powershell
.\scripts\bin\harness-cli.exe
```

Generate context, validate, trace và verify:

```bash
scripts/bin/harness-cli context --story US-001
scripts/bin/harness-cli arch-check --story US-001
scripts/bin/harness-cli story update --id US-001 --unit 1 --integration 1 --e2e 0 --platform 0
scripts/bin/harness-cli trace --summary "Completed US-001" --story US-001 --outcome completed
scripts/bin/harness-cli story verify US-001
```

`story verify` chạy proof command đã cấu hình và enforce governance gate.

Một story chưa được xem là xong cho đến khi cả proof và gate đều pass.

---

## Workflow Cốt Lõi

Mọi task nghiêm túc đều đi theo cùng một đường:

```text
Human request
  → intake classification
  → story packet
  → context pack
  → implementation
  → validation proof
  → trace
  → story governance gate
  → governance dashboard
```

Các command thường dùng:

```bash
scripts/bin/harness-cli intake --summary "<work>" --lane normal --story US-001
scripts/bin/harness-cli context --story US-001
scripts/bin/harness-cli arch-check --story US-001
scripts/bin/harness-cli story verify US-001
scripts/bin/harness-cli trace --summary "<what happened>" --story US-001 --outcome completed
scripts/bin/harness-cli governance report --output .harness/reports/governance-report.json
scripts/bin/harness-cli governance dashboard --report .harness/reports/governance-report.json --output .harness/dashboard/index.html
```

Agent nên đọc các file này trước khi implementation:

```text
AGENTS.md
docs/HARNESS.md
docs/FEATURE_INTAKE.md
docs/ARCHITECTURE.md
docs/CONTEXT_RULES.md
```

Dùng command này để xem proof state hiện tại:

```bash
scripts/bin/harness-cli query matrix
```

---

## Governance Dashboard

Xuất governance evidence:

```bash
scripts/bin/harness-cli governance report \
  --output .harness/reports/governance-report.json

scripts/bin/harness-cli governance dashboard \
  --report .harness/reports/governance-report.json \
  --output .harness/dashboard/index.html
```

Mở file này trên máy local:

```text
.harness/dashboard/index.html
```

Dashboard hiển thị:

<table>
  <tr>
    <th>Phần</th>
    <th>Nội dung</th>
  </tr>
  <tr>
    <td>Story proof</td>
    <td>Trạng thái unit, integration, E2E, platform và custom proof.</td>
  </tr>
  <tr>
    <td>Governance gate</td>
    <td>Pass/fail state cho từng story.</td>
  </tr>
  <tr>
    <td>Release evidence</td>
    <td>Kết quả trusted distribution verification.</td>
  </tr>
  <tr>
    <td>Friction</td>
    <td>Blocker và tín hiệu cải thiện workflow.</td>
  </tr>
  <tr>
    <td>Maturity</td>
    <td>Tóm tắt governance maturity hiện tại.</td>
  </tr>
</table>

Dashboard là static HTML và không dùng external assets.

---

## Trusted Distribution

HI-OS publish prebuilt Harness CLI binaries và SHA256 assets cho:

```text
macOS arm64
macOS x64
Linux x64
Linux arm64
Windows x64
```

Kiểm chứng public release chain:

```bash
scripts/bin/harness-cli release verify --version 0.7.0
```

Trên Windows:

```powershell
.\scripts\bin\harness-cli.exe release verify --version 0.7.0
```

`release verify` kiểm tra:

```text
release metadata
5 platform binaries
5 binary SHA256 files
production payload ZIP
production payload SHA256
host binary version
smoke command
```

Lỗi network hoặc GitHub availability là:

```text
inconclusive
```

không phải:

```text
pass
```

Default public origin là:

```text
ntu254/Harness-Intelligence-OS
```

Origin này được cấu hình trong:

```text
harness-release.toml
```

và được align với tracked identity trong:

```text
hios.toml
```

---

## Production-Clean Payload

Để phân phối production, HI-OS build một platform-neutral payload ZIP.

Payload này chứa installer source và operating contract, nhưng không chứa:

```text
Rust source
historical story packets
archived plans
runtime evidence
local harness.db
.harness runtime files
```

Build và verify:

```bash
bash scripts/build-production-payload.sh --version 0.7.0
python scripts/verify-production-payload.py --version 0.7.0 --source-check
```

File được tạo nằm trong:

```text
dist/
```

Upload lên public release vẫn thuộc bước release hardening.

---

## CodeGraph Và NotebookLM Evidence

HI-OS dùng file-based provider boundaries.

External providers **không được ghi trực tiếp vào Harness SQLite**.

### CodeGraph impact evidence

```bash
scripts/bin/harness-cli codegraph impact \
  --story US-001 \
  --mode changed-files \
  --changed-files .harness/context/changed-files.txt
```

### NotebookLM grounded brief evidence

```bash
scripts/bin/harness-cli notebooklm brief \
  --story US-001 \
  --notebook <notebook-id-or-alias> \
  --query "Find citation-backed context for this story."
```

Provider rules:

<table>
  <tr>
    <th>Điều kiện</th>
    <th>Kết quả</th>
  </tr>
  <tr>
    <td>Provider unavailable</td>
    <td><code>inconclusive</code></td>
  </tr>
  <tr>
    <td>Network/session unavailable</td>
    <td><code>inconclusive</code></td>
  </tr>
  <tr>
    <td>Missing citations</td>
    <td><code>fail</code></td>
  </tr>
  <tr>
    <td>Malformed provider output</td>
    <td><code>fail</code></td>
  </tr>
  <tr>
    <td>Valid artifact + successful ingest</td>
    <td><code>pass</code></td>
  </tr>
</table>

Harness không bao giờ lưu:

```text
Google credentials
cookies
tokens
browser profiles
provider session files
```

---

## Sơ Đồ Repository

```text
AGENTS.md                         stable agent entrypoint
docs/HARNESS.md                   human-agent operating model
docs/FEATURE_INTAKE.md            risk lanes and intake rules
docs/ARCHITECTURE.md              architecture boundary rules
docs/CONTEXT_RULES.md             what agents should read and when
docs/agents/                      Cursor, Claude Code, and Codex packs
docs/adoption/                    first-run adoption walkthroughs
docs/examples/                    end-to-end example workflows
docs/troubleshooting.md           installer, provider, gate, and dashboard fixes
docs/COMMAND_COOKBOOK.md          grouped command examples
docs/demo/                        example transformation from idea to story
docs/archive/                     historical phase plans and early specs
docs/stories/                     story packets and validation evidence
docs/decisions/                   durable decisions
docs/schemas/                     artifact and report schemas
packaging/production-include.toml production payload contract
scripts/bin/harness-cli           installed CLI path in target projects
scripts/README.md                 command and installer details
```

---

## Nên Bắt Đầu Từ Đâu

<table>
  <tr>
    <th>Nhu cầu</th>
    <th>File nên đọc</th>
  </tr>
  <tr>
    <td>Setup từ clean clone</td>
    <td><code>docs/adoption/clean-clone-walkthrough.md</code></td>
  </tr>
  <tr>
    <td>Full agent workflow</td>
    <td><code>docs/examples/full-agent-workflow.md</code></td>
  </tr>
  <tr>
    <td>Hướng dẫn cho Codex</td>
    <td><code>docs/agents/codex.md</code></td>
  </tr>
  <tr>
    <td>Hướng dẫn cho Claude Code</td>
    <td><code>docs/agents/claude-code.md</code></td>
  </tr>
  <tr>
    <td>Hướng dẫn cho Cursor</td>
    <td><code>docs/agents/cursor.md</code></td>
  </tr>
  <tr>
    <td>Troubleshooting</td>
    <td><code>docs/troubleshooting.md</code></td>
  </tr>
  <tr>
    <td>Ví dụ command</td>
    <td><code>docs/COMMAND_COOKBOOK.md</code></td>
  </tr>
  <tr>
    <td>Chi tiết installer</td>
    <td><code>scripts/README.md</code></td>
  </tr>
</table>

Xem command details:

```bash
scripts/bin/harness-cli help
```

---

## Milestone Hiện Tại

<table>
  <tr>
    <th>Version</th>
    <th>Chủ đề</th>
  </tr>
  <tr>
    <td>v0.1</td>
    <td>Context-grounded auto intake</td>
  </tr>
  <tr>
    <td>v0.2</td>
    <td>Blocking governance gate</td>
  </tr>
  <tr>
    <td>v0.3</td>
    <td>Trusted distribution and evidence trail</td>
  </tr>
  <tr>
    <td>v0.4</td>
    <td>MCP artifact contracts and provider adapters</td>
  </tr>
  <tr>
    <td>v0.5</td>
    <td>Structured friction learning loop</td>
  </tr>
  <tr>
    <td>v0.6</td>
    <td>Governance report, maturity summary, and static dashboard</td>
  </tr>
  <tr>
    <td>v0.7</td>
    <td>Adoption Ready</td>
  </tr>
</table>

HI-OS v0.7 là adoption line đã hoàn thiện:

```text
installable
understandable
debuggable
production-clean
publicly verifiable
```

---

## Đóng Góp

Các đóng góp hữu ích gồm:

```text
test clean clone walkthrough trên máy mới
cải thiện docs cho người dùng lần đầu
thêm ví dụ project thật
báo cáo agent failure do thiếu repo context
cải thiện validation pattern cho nhiều stack khác nhau
so sánh hành vi giữa Codex, Claude Code, Cursor và các agent khác
```

Xem:

```text
CONTRIBUTING.md
```

---

## Mô Tả Ngắn

**HI-OS là một lớp vận hành repository dành cho Codex, Claude Code, Cursor và các coding agents khác: intake, context packs, story gates, release verification, governance dashboards và durable traces.**
