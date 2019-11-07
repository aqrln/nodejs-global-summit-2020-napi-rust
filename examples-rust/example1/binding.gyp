{
  'targets': [
    {
      'target_name': 'example1',
      'sources': ['manifest.c'],
      'libraries': [
        '../../target/release/libnapi_example1.a',
      ],
      'include_dirs': [
        '../napi/include'
      ]
    }
  ]
}
