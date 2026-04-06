import 'dart:js_interop';

import 'package:web/web.dart' as web;

String resolveWebApiBaseUrl() {
  web.window.location.origin.toJS.toDart;
  return '/api';
}
