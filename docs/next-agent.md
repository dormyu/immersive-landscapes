# Продолжение Immersive Landscapes —0.3.1 / DHdormyu.6

Рабочий мод в активной разработке, автор dormyu (SRC-018).
Актуальные [проверки стыка воды](dh-water-parity-2026-11-04.md) и [первой0.3.1](validation-0.3.1.md).
DormLab: Horizon f42ef42, **runtime5b599bb2**, IL0.3.1/v2/descriptor6,
DH0.1.0+dormyu.6, официальный Zoom2.0.0. Генерация в этой итерации не менялась;
IL native-бандл и snapshotaaa4a0b сохранены. DH built code0807fc6/sourcec1b70df.

Исправлены высота воды, GPU-готовность, пространственные маски near и старые
результаты работников. В сравнительной сцене чёрная полоса исчезла (723→0);
разница детализации дна местами остаётся. Кэш water-volume-v2 совместим,
полного сброса не требуется. Для DH можно открыть старый мир; для новой
генерации — новый тип Immersive Landscapes. Мирыv4/v5/v6 не мигрировать молча.

23 теста/Clippy, UI/save/старые рецепты, два клиента и установленный Prism
проверены. 3779 файлов личных saves и настройки сохранены, control off.
Короткий near32/DH128 Medium1080p/cap120 flight119.62FPS/max14.77ms —
не приёмка тяжёлой сборки, cold near32, длительного исследования или всех сцен.

Следующее: оставшиеся near/far различия дна/материалов, затем гидрология
с выходами/устьями/дельтами и экотоны, сохраняя Minecraft-масштаб и лес/биомы.
Не начинать survival/классы/DormCore и не возвращать bare height-only или×4 горы.
Точную текущую поставку читать в ../RustCore/packs/DormLab/rustcore/native-build.json.
QA/откат: ../RustCore/qa/dh-water-parity-2026-11-04/before-near-parity-install.
Все файлы/temp/cache E:, Enter-RustDev.ps1, stable, одна Cargo job.
Сохранить личные cap260/VSync off/near32; не сбрасывать к историческим120/on.

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
