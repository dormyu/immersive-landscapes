# Следующая модель: реализовать генератор в Horizon

Обновлено 2026-11-02. **Сначала прочитать это, затем implementation-plan.ru.md.**
Автор уточнил: только реалистичные горы, реки/водоёмы и плавные переходы биомов,
с DH и хорошим перформансом. Survival/прогрессия/классы/отдельный движок не входят.
Прежняя оценка до 100 часов и план G0–G7 заменены четырьмя этапами G0–G3.
Последний запрос автору выполнен как передача плана; текущий агент закончил работу.

## Первый конкретный шаг

Сделать native bundle и подключить **уже написанный** `Landscape::sample` к
`ChunkGenerator` и `LodSampler`, используя штатные `Persistent` и `Streaming`.
Первый результат — новый игровой мир с нашими блоками и DH. Не начинать с нового
плана, переписывания DH или исследования всех модов. Хуки-кандидаты и критерии
проверки перечислены в [плане](implementation-plan.ru.md).

## Точное исходное состояние

- Репозиторий: `E:/Games/Minecraft/immersive-landscapes`, GitHub dormyu/immersive-landscapes.
  Автор коммитов dormyu, verified local identity уже настроена; global не менять.
- Terrain library уже есть: хребет/предгорья, климат, сток/озёра/притоки, кандидаты
  дельт, indexed sampler. 11 тестов прошли в сохранённом QA. **Игрового адаптера нет.**
- DH: `E:/Games/Minecraft/RustCore/development/distant-horizons`, local `2ab4f24`.
  Установлен в DormLab, 16 тестов и solo/two-client checks выполнены. Не портировать снова.
- Engine pin `c9d474dd9116e84c243f8992d75c11314df56f06`; source:
  `E:/Games/Minecraft/RustCore/reports/terrain-research-2026-11-02/nightly-source-c9d474d/horizon-c9d474d`.
- Mixins pin `0d6b103c70633d8fd9443d8d0acd3aadaacedb06`.
- Lab: `E:/Games/Minecraft/RustCore/packs/DormLab`; `rustcore/native-build.json`.
- Rollback: `E:/Games/Minecraft/RustCore/qa/dormlab-before-dh-2026-11-02`.
- QA/status: `E:/Games/Minecraft/RustCore/reports/development-2026-11-02/WORK-IN-PROGRESS.md`.
  Там native-control gotchas, private QA paths, предыдущие проверки и известные ограничения.
- Цель: баланс красоты/затрат, идеал 32 near chunks / 120 FPS, DH дальше, слабым ПК
  настройки. До ~10x дольше подготовка допустима. Результат FPS ещё не доказан.
- PC i5-9400F 6C/6T, RTX2060, ~16GiB, desktop1080p144Hz. E HDD, всё хранить на E.
  Не выводить VRAM по усечённому WMI AdapterRAM. Пользователь разрешил сборки до
  ручной перезагрузки; самому не перезагружать и не менять подкачку.

## Использование исходников референсов

FreeTerraForged `1.21.1_unstable`/LICENSE проверены через GitHub 2026-11-02:
MIT, отдельный `Erosion.java` исключён под MPL-2.0. Полного совместимого Horizon
билда этого генератора у нас нет. Брать конкретные полезные алгоритмы после pin
и проверки header/license, с attribution; не переписывать Java-инфраструктуру мода.
Другие референсы и ограничения указаны в плане и CREDITS.

## Команды и обязательные ограничения

Сначала parent AGENTS.md и RustCore/AGENTS.md. Перед изменением engine stage читать
его AGENTS.md/CONTRIBUTING.md. Старые миры/аккаунты/clean baseline сохраняются.

```powershell
. E:\Games\Minecraft\RustCore\development\scripts\Enter-RustDev.ps1
$env:CARGO_TARGET_DIR = 'E:\Games\Minecraft\immersive-landscapes\target'
cargo fmt --all -- --check
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
```

Сборочный путь DH уже имеет `tools/horizon.py stage --with <IL bundle>` и
`build --with <IL bundle>`. Но **Run-GuardedBuild.py сейчас не передаёт --with**:
сначала добавить forwarding/combined wrapper, сохранив guard C>=6GiB, jobs=1,
TEMP/TMP/Cargo/Rustup/targets на E и остановку только своего дерева процессов.
Папка нашего bundle `mixins/immersive_landscapes` ещё не создана.
Нельзя руками менять `target/stage` или запускать две полные сборки параллельно.

Обычные API/native hooks использовать первыми. `make_streaming` и `level_shapes`
проверить planner/compiler; это пока кандидаты. `lod_source_in` уже берёт источник
из streaming. `Chunk::with_layout`, `Section::from_blocks`, `share_uniform` позволяют
не писать хранение/освещение/мешер заново. Сам код мода остаётся Rust.

Не публиковать assets/prepared resources/accounts/control tokens/worlds. MIT IL и
LGPL DH сохраняют свои notices. Сообщения третьим лицам/claim issues не отправлять.
Работать CLI/native control; `bench`/`stats`/`fly ... raw` уже доступны. Offscreen —
для smoke-tests; итоговые FPS только в видимом окне с записанными настройками.
Короткий отчёт после реального результата. Завершённость не подменять обещаниями.
