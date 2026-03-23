# سجل التغييرات

توثيق جميع التغييرات الملحوظة في هذا المشروع.

الصيغة مبنية على [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
والمشروع يتبع [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [غير مُصدّر]

## [0.2.0] — غير مُصدّر

### مضاف

- **واجهة برمجية Async Streaming** — `client.stream_domain(names) -> ReceiverStream<DomainEvent>` و `client.stream_ip(addresses) -> ReceiverStream<IpEvent>`؛ تنتج النتائج مع وصولها دون تخزين الدفعة الكاملة
- **Back-pressure** — قناة `tokio::sync::mpsc` محدودة؛ `StreamConfig.buffer_size` تتحكم في السعة (افتراضي 32)؛ المُرسلون يحجبون عندما يتأخر المستهلك — لا نمو ذاكرة غير محدود على نطاق واسع
- **تعددات `DomainEvent` / `IpEvent`** — متغيرات `Ok(DomainResponse)` / `Err(RdapError)`؛ المتغيرات الكبيرة مصندوقة لقمع `clippy::large_enum_variant`
- **تكوين Connection pool** — `ClientConfig.reuse_connections: bool` (افتراضي `true`) و `ClientConfig.max_connections_per_host: usize` (افتراضي `10`)
- **Go binding** (`rdapify-go`) — دالف cgo أولية في `bindings/go/rdapify.go` حول هدف `cdylib`؛ تعريض 5 دوال متزامنة (`domain` و `ip` و `asn` و `nameserver` و `entity`) تقود داخلياً تشغيل `tokio`؛ رأس C `rdapify.h` مع تعليقات توثيق كاملة؛ تم إضافة وظيفة فحص بناء CI إلى `.github/workflows/ci.yml`
- **معيار Streaming** — `benches/streaming.rs` (Criterion) قياس الإنتاجية لـ `stream_domain` تحت حمل متزامن

### الاختبارات

- تدفق يأخذ جميع النتائج بالترتيب
- الخطأ في عنصر واحد لا يلغي العناصر المتبقية
- الإلغاء في منتصف Stream (إسقاط المستقبل) ينهي المُرسل برفق

## [0.1.3] — غير مُصدّر

### مضاف

- **`domain_available()`** — `client.domain_available(name) -> Result<AvailabilityResult>` يتحقق ما إذا كان النطاق متاحاً للتسجيل؛ إرجاع `available: true` في HTTP 404 من السجل، `available: false` مع `expires_at` للنطاقات المسجلة
- **نوع `AvailabilityResult`**: `{ domain: String, available: bool, expires_at: Option<String> }` — مُصدَّر من الـ API العام
- **`ClientConfig.custom_bootstrap_servers: HashMap<String, String>`** — تجاوز TLD → عنوان URL خادم RDAP مخصص، استشير قبل بحث Bootstrap IANA
- 11 اختبار تكامل جديد: مسار سعيد `domain_available`، 404 → متاح، انتشار الخطأ، نطاق/IP/ASN غير صحيح، ذاكرة مؤقتة معطلة، max_attempts=1، خادم bootstrap مخصص

## [0.1.2] — 2026-03-21

### تم تغييره

- **إعادة تسمية**: تم إعادة تسمية Node.js binding من `@rdapify/core` → `rdapify-nd` على npm
- **إعادة تسمية**: تم إعادة تسمية Python binding من `rdapify` → `rdapify-py` على PyPI؛ اسم استيراد Python تم تغييره من `rdapify` → `rdapify_py`
- **الأداء**: ربط `rdapify-nd` napi الآن يستخدم `OnceLock<RdapClient>` على مستوى الوحدة — يلغي overhead بناء العميل لكل استدعاء

### تم إصلاحه

- **CI**: تم إصلاح هدف `aarch64-apple-darwin` المكرر في مصفوفة بناء napi `bindings.yml` (كان مدرجاً أيضاً في `napi.triples.defaults`)

### التوثيق

- أضيفت أمثلة استخدام كاملة لـ `rdapify-nd` (Node.js) و `rdapify-py` (Python) في README

## [0.1.1] — 2026-03-20

### مضاف

- دعم معيار DNS over HTTPS
- تحسينات موثوقية الاتصال

## [0.1.0] — 2026-03-15

### مضاف

- **عميل RDAP الأساسي** — استعلامات Domain و IP و ASN
- **IANA Bootstrap** — اكتشاف خادم تلقائي
- **حماية SSRF** — حجب IP الخاصة
- **ذاكرة مؤقتة** — ذاكرة مؤقتة داخل الذاكرة مع TTL

---

آخر تحديث: مارس 2026
