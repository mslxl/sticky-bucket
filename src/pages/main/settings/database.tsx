import PrefFile from "@/components/pref/pref-file";
import { useAtom } from "jotai";
import cfg from '@/store/settings'

export default function DatabaseSettingSection() {

    const [stickerDataFolder, setStickerDataFolder] = useAtom(cfg.database.stickerDir)


  return (
    <ul>
      <li className="border-t py-2">
        <PrefFile
          leading="Sticker Data Folder"
          value={stickerDataFolder}
          open
          option={{
            title: 'Open sticker data folder',
            directory: true,
            recursive: false
          }}
          onValueChange={setStickerDataFolder}
          description={() => <span>Where stickies and databse located in.<br/><em>*Effects after restart</em></span>}
        />
      </li>
    </ul>
  );
}
