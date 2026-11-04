# Current checkpoint — DH dormyu.7 / IL 0.3.1

DormLab client `7292006be9ccf7629d3e6052f799244665f413a9f02bfb9ae7ad4fd1cbc32ca4`, Horizon f42ef42, IL0.3.1/v2/descriptor6,
DH0.1.0+dormyu.7, unchanged official Zoom2.0.0. Working version, active development.
Terrain recipe unchanged; IL native package/source snapshotaaa4a0b retained.
DH built code593671e; full source commit recorded in native-build.json.

Bounded near-boundary detail in Medium/High/Extreme and native biome tint parity;
previous water/GPU/exact mask/stale-worker fixes retained. Low/Lowest unchanged.
[Current comparison and measured cost](dh-boundary-detail-2026-11-04.md).
26 tests/Clippy, solo/UI/save/old recipes, two Rust clients and installed Prism pass.
5127 owner save files/settings preserved; control off. Teams left running.
Short warmed near32/DH128 Medium/cap120 flight 119.53FPS,
P99 9.41/max 18.43ms is not heavy-pack acceptance.

Next: owner review of this exact installed build/world, remaining plant/material/detail
transitions, then hydrology/outlets/deltas and gradual vegetation at Minecraft scale.
Do not repeat the v4 acceptance mistake or silently migrate recipes. No survival/classes,
DormCore framework, bare height-only prototype or fourfold mountain amplification.
Future structures target eventually approved versioned IL geography.

QA/rollback: ../RustCore/qa/dh-boundary-detail-2026-11-04/before-boundary-detail-install.
Exact delivery: ../RustCore/packs/DormLab/rustcore/native-build.json.
Native /dh commands must pass through client chat (T/type/Enter). The control
command endpoint executes on the server; the first quality-switch harness failure
and its timings remain in reports/perf-harness-first, followed by a corrected run.
All files/temp/cache on E:, Enter-RustDev.ps1, stable, one Cargo job.
Preserve personal cap260/VSync off/near32; never close Teams to make room.

Below is historical handoff content; current versions above take precedence.

---

Ниже сохранён исторический handoff0.3.0-dev.2. Его «текущие» версии и цифры
относятся к предыдущим checkpoint, а не к0.3.1.

# Продолжение Immersive Landscapes

3 ноября 2026. Автор dormyu, репозиторий dormyu/immersive-landscapes.

## Текущий курс

Обычный Minecraft с природной логикой рельефа (Obsidian SRC-015). Только
генерация, без survival-систем, классов и нового движка. Не возвращать ×4
высоты или голый height-only прототип. Полный Epic больше не default новых миров.

0.3.0-dev.2 / descriptor5 / vanilla-natural-v1: шесть оригинальных density-функций
поверх Horizon f42ef42 /26.3. Общие поля хребтов и долин используются также
выбором биомов; равнинные долины расширены относительно горных, дно внутренних
русел защищено от чрезмерного понижения. Штатные климат, континенты, высота мира,
определения биомов, деревьев, пещер, руд и структур сохранены.
[Устройство и ограничения](vanilla-natural.md).

Прежняя приёмка v4 была ошибкой ассистента: автор хвалил обычный мир в
DormLab-EpicReference с IL и DH off. Не восстанавливать эту приёмку из старых
отчётов. [Разбор](feedback-correction.md). Новая реализация тоже требует отзыва.

## Активный код

`tools/prepare_vanilla_natural.py` воспроизводит JSON и SHA-256.
`tools/native.py generate/package` проверяет хэши и LF, встраивает данные.
Native HEAD WorldRequest::prepare_world устанавливает рецепт ДО регистров,
также на Rust-сервере. Исходный vanilla offset сохраняется как приватный alias
из ресурсов владельца: не публиковать его в Git или hnmod.

Меню: **Immersive Landscapes**. Слот сохранения minecraft:normal плюс свой
marker/datapack. Vanilla-миры пропускаются; старые v4/Epic и новые v5 сохраняют
свой рецепт. v1–v3 требуют архивной сборки. Выключение IL возвращает Default
для новых миров. Внешний override обязан иметь собственный IL-PRESET.json.
`src/` — историческое исследование, не нынешний генератор чанков.

## Доказательства и доставка

Точный статус: `../RustCore/packs/DormLab/rustcore/native-build.json`.
Журнал: `../RustCore/reports/development-2026-11-03/WORK-IN-PROGRESS.md`.
QA текущего бинарника: `../RustCore/qa/dh-lifecycle-2026-11-03`.
[Проверки0.3.0-dev.2](validation-0.3.0-dev.2.md). Старые проверки ниже — история
vanilla-natural-v1 на c9; не подменять ими readback установленного f42-билда.
Прочитать реальные отчёты перед утверждением, что проверка пройдена.

- verify_natural_world.py: RU/EN UI, создание v5, точный рецепт, сохранение
  блока, отключение IL, неизменность копии v4.
- verify_natural_multiplayer.py: сервер и два Rust-клиента, общий блок,
  decode_errors и геометрия DH. Near2/loopback — не нагрузочная проверка.
- verify_natural_view.py: одинаковые seed/камера, vanilla, IL без/с DH128.
  Offscreen1080p, near16, cap120, VSync off. Не выдавать за видимый near32/120.
- verify_prism_natural.py: установленная DormLab через Prism, control off;
  QA-мир вне пользовательских saves, восстановление настроек и сверка saves.

Три native-пробы: 4225 точек / квадрат4096, seeds -6712665269073713615
(обычный мир владельца), 8675309, 482193. Это ограниченные выборки.

## Следующая работа

1. Авторский отзыв о конкретном новом мире и точных настройках. Не объявлять
   качество или плавность принятыми самостоятельно.
2. Дальняя вода/материалы, заполнение и переход near/DH. Ненулевой LOD mesh
   не доказывает отсутствие дыр или совпадение поверхностей.
3. Сеть водосборов, истоков/устьев/дельт; шумовые долины её не обеспечивают.
   Переходы растительности также открыты.
4. Холодные/прогретые маршруты, near32 и запас под тяжёлую сборку.
5. После выбора основы — авторские структуры. V5 ещё не утверждён как
   окончательная география; DormCore пока не нужен.

Всё на E:, Enter-RustDev.ps1, stable toolchain, одна Cargo job; без игры
во время полной сборки. DH — отдельная адаптация dormyu.4 / код4226836, mixinsddd9700 + patchd4af4d8.
Новый QA: verify_native_functional.py, verify_native_network.py, native_view.py,
near_only.py, install_native.py, verify_native_prism.py. Runtime SHA5dd3d0cd.
Установку подтверждать только по native-build.json и Prism readback.
Некоторые серые обрывы подтверждены в настоящих near32-чанках без LOD;
нужно отдельно проверять поверхность и сами дальние сетки. 32/120 не принято.
Читать AGENTS; сохранять миры/аккаунты/чистую RustCore. Не читать токены.
CLI/native control только в тестовых копиях. После правок: сборка → QA →
откат → установка → реальный Prism → readback → Obsidian и проверка ссылок.
История v4: validation-0.2.0-dev.1.md и Git, не результаты проверки v5.

## Latest local lab supplement

[Official Zoom, restored near32 profile and current checks](dormlab-zoom-2026-11-03.md).
Current native hashes/reports come from the lab manifest; old near16 results are historical.
