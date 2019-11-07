{
  'targets': [
    {
      'target_name': 'example2',
      'sources': ['manifest.c'],
      'libraries': [
        '../../target/release/libnapi_example2.a',
      ],
      'include_dirs': [
        '../napi/include'
      ]
    }
  ]
}
