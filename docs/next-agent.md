# Продолжение Immersive Landscapes

Актуально на 2026-11-02. Читать сначала этот файл, затем
[план](implementation-plan.ru.md) и [результаты референсов](reference-findings.md).
Автор: dormyu. Репозиторий: https://github.com/dormyu/immersive-landscapes.

## Что уже сделано

- G0 выполнен: native bundle подключает Rust core к ChunkGenerator, Persistent,
  Streaming и LodSampler. Это мод Horizon, отдельный движок не создаётся.
- Билд G0 собран за 9m54; первая версия создала игровой мир с DH и сохранила
  emerald block после закрытия/открытия. Native-код на main начиная с 24bb47e.
- Вторая версия (0.1.0-dev.2, descriptor2, grid513/cell64) собрана за 7m03,
  запущена на двух seeds. Двенадцать core tests и clippy прошли.
- Автор отверг картинку: плоские склоны, лысые горы, неестественная композиция.
  G1 НЕ принят. Не подкручивать Gaussian belt бесконечно и не выдавать FPS за красоту.
- Статические visible1080p near32/DH256 тесты упрощённого прототипа превышали120FPS.
  Это не результат для красивого нового генератора, движения или свежих чанков.
- Старый тест near32 с29.7FPS был ограничен unfocused cap и помечен invalid.
- DH в DormLab — ранее проверенный порт/адаптация, не портировать заново.
  Новый fog fix с17 core tests уже в QA-бинарнике, но ещё не установлен в DormLab.

## Epic Terrain: реальный результат последней проверки

- Автор попросил Tellus и Epic Terrain; предпочитает procedural world по seed.
  Earth DEM допустим, но не выбран обязательной основой.
- Epic Terrain MIT source08507bd3d606ad440b40a5fa726e0098d7309347 находится в
  ../RustCore/development/references/Epic-Terrain и остаётся неизменным.
- Его 1.21 pack адаптирован в26.3 через штатный Rust horizon-port-pack и наш
  tools/prepare_epic_reference.py. Восемь узких compatibility changes задокументированы.
  LICENSE и PROVENANCE.json сохраняют автора wonderfulaichen.
- Реальная загрузка в Horizon удалась; новый мир Epic-Reference-v2 находится
  в независимом QA profile epic-probe. IL отключён: это честный чужой референс,
  а не переименованный в IL генератор. После последних fixes unsupported_features=[].
- Клиент прочитал stone на -3072/347/-2560: высота выше ванильной действительно работает.
- Native diagnostic:10 выбранных сухих колонок совпали по высоте с настоящими
  proto chunks. В речных точках расхождение: например8192/-3072 настоящая водаY105,
  far sampleY108 иwater=None. Все данные в reports/epic-sampler-differential.json.
- Epic скульптит русло и воду через material rules, а Horizon far shortcut
  применяет их только к верхним8 блокам и пропускает slope rules. Не считать
  повышение DH quality исправлением этих данных. Причины всех артефактов ещё не установлены.

## Следующее действие

Использовать проверенные Epic noise/spline/surface recipes как базу сравнения и
источник конкретных настроек для нового IL. Реально использованные файлы указывать
как derived с MIT notice. Не просто переименовывать весь Epic pack.
Сохранить собственную связную гидрологию: шумовые реки Epic не доказывают сток,
дельты и согласованные истоки/устья. Сначала проверить near/far water/materials
на одних координатах, затем оценивать цельный красивый мир.

Отдельно исправить IL LOD: Surface-only fallback заполняет подземелье верхним
материалом и не содержит деревьев. Нужны слои и тот же canopy, что в чанках.
У DH local adapter пока width1 per point; remote уже использует tiles/grid.
Батчирование local делать с правильным half-stride: runtime уже передаёт центр,
а Horizon grid принимает угол и сам добавляет width/2. Не сдвинуть дважды.

## Сборка и ограничения

- Всё на E:, читать parent AGENTS.md и ../RustCore/AGENTS.md. Профиль/QA:
  ../RustCore/qa/immersive-landscapes-2026-11-02; актуальный журнал:
  ../RustCore/reports/development-2026-11-02/WORK-IN-PROGRESS.md.
- Перед Rust: source ../RustCore/development/scripts/Enter-RustDev.ps1;
  RUSTUP_TOOLCHAIN=stable. Cargo jobs1; никаких параллельных полных engine builds.
  Не перезагружать ПК. Полные сборки до ручной перезагрузки разрешены автором.
- Engine c9d474dd9116e84c243f8992d75c11314df56f06 / MC26.3.
  Mixins0d6b103 + local boundary-HEAD patch19140348cf112441a6d535a11332b3678ac70474.
  tools/native.py выбирает patched tool; guard передаёт --with и пишет логи на E.
  См. native-build.md. Не редактировать pristine/stage вручную.
- Rust client и Rust dedicated server. Соло обязательно; два клиента ещё нужно
  проверить именно с новым генератором. Survival, классы и DormCore не добавлять.
- Цель32 настоящих near chunks/120FPS, DH отдельно; слабым ПК настройки,
  запас будущей тяжёлой сборке. Долгая предгенерация допустима.
- DH distance допустим64/128/256/512/1024. Значения96/192 молча падают в64.
- Native ctl вместо computer-use. Control off в доставленных экземплярах;
  control sockets/private auth никогда не читать/коммитить. FPS мерить в видимом
  окне с inactivityFpsLimit=minimized, без одновременной компиляции.
- Старые миры, аккаунты и чистый RustCore не менять. DormLab пока DH-only;
  визуально отвергнутый IL туда не устанавливался. Откат перед установкой в QA/before-install.
- FreeTerraForged sourceefc7565b6e19bf5ec5ee5abcb0bf9a2c6072a70b тоже скачан.
  MIT, кроме исключённого MPL tile/filter/Erosion.java. Не читать/переносить его;
  noise/module/Erosion.java отдельно MIT. Прямого переноса FTF кода пока нет.
- Tellus LGPL: изучен подход DEM/climate/canopy/direct LOD, код в MIT IL не копировался.

## Когда можно закончить

G1: автор принимает вид гор/воды/биомов; G2: representative frame times и budget,
а не только статичный кадр; G3: pinned artifacts/credits/rollback, solo save/restart,
два Rust-клиента, обычный запуск из Prism. Готовность мода пока не достигнута.
