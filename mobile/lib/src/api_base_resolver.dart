import 'api_base_resolver_native.dart'
    if (dart.library.js_interop) 'api_base_resolver_web.dart'
    as impl;

String resolveWebApiBaseUrl() => impl.resolveWebApiBaseUrl();
