---
title: Lucky Framework: upload a file with Shrine while keeping the original filename
slug: lucky-fileupload-custom-filename-shrine
date: 2021-02-05 09:00:00+00:00
tags: crystal, beginner
category: 
link: 
description: 
type: text
---

On the [Lucky's website](https://luckyframework.org/guides/handling-files/file-uploads), the way to go to upload files is to use [Shrine.cr](https://github.com/jetrockets/shrine.cr). The problem is that, by default, `Shrine.cr` is not using the original name of the file to store it, but instead uses a generated id. And of course, for my particuliar use case (serving files directly through Nginx) I need the file to be stored with it's original filename.

<!-- TEASER_END -->

## Overriding the `generate_location` method

It's pretty simple to achieve (once you know it 😅), the only thing you have to do is to override the `generate_location` method of the `Shrine` class.

```crystal
class FileImport::AssetUploader < Shrine
  def generate_location(io : IO | UploadedFile, metadata, **options)

    if metadata.has_key?("filename")
      metadata["filename"].to_s
    else
      super(io, metadata, **options)
    end

  end
end
```

Then, you just have to replace the default uploader

```crystal
result = Shrine.upload(File.new(pic.tempfile.path), "store", metadata: { "filename" => pic.filename })
```

with your custom one:


```crystal
result = FileImport::AssetUploader.upload(File.new(pic.tempfile.path), "store", metadata: { "filename" => pic.filename })
```

And you're done! 🎉
