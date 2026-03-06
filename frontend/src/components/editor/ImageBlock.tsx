import { useRef } from 'react';
import type { Block, ImageBlockData } from '../../lib/editor/types';

interface ImageBlockProps {
  block: Block;
  onChange: (b: Block) => void;
  locked?: boolean;
}

export function ImageBlock({ block, onChange, locked }: ImageBlockProps) {
  const data = block.data as ImageBlockData;
  const fileRef = useRef<HTMLInputElement>(null);

  function handleFile(file: File) {
    const reader = new FileReader();
    reader.onload = (e) => {
      const src = e.target?.result as string;
      onChange({ ...block, data: { ...data, src } });
    };
    reader.readAsDataURL(file);
  }

  if (!data.src) {
    return (
      <div
        className="border-2 border-dashed border-gray-300 rounded-lg p-8 text-center cursor-pointer hover:border-blue-400 transition-colors"
        onClick={() => !locked && fileRef.current?.click()}
      >
        <p className="text-gray-500 text-sm">Click to upload image</p>
        <input
          ref={fileRef}
          type="file"
          accept="image/*"
          className="hidden"
          onChange={(e) =>
            e.target.files?.[0] && handleFile(e.target.files[0])
          }
        />
      </div>
    );
  }

  return (
    <div className="my-2" style={{ textAlign: block.meta.align }}>
      <img
        src={data.src}
        alt={data.alt}
        style={{ maxWidth: data.width, maxHeight: data.height }}
        className="inline-block"
      />
      {!locked && (
        <input
          value={data.alt}
          onChange={(e) =>
            onChange({ ...block, data: { ...data, alt: e.target.value } })
          }
          className="block w-full text-xs text-gray-500 text-center mt-1 outline-none"
          placeholder="Alt text"
        />
      )}
    </div>
  );
}
