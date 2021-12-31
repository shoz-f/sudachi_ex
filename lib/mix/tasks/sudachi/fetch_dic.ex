defmodule Mix.Tasks.Sudachi.FetchDic do
  use Mix.Task
  import Mix.Generator
  
  @shortdoc "Download Sudachi resources."
  
  def run(argv) do
    dest_dir = Application.app_dir(:sudachi, "priv/resources")
    [
      "wget http://sudachi.s3-website-ap-northeast-1.amazonaws.com/sudachidict/sudachi-dictionary-20210802-core.zip",
      "unzip sudachi-dictionary-20210802-core.zip",
      "mkdir -p #{dest_dir}",
      "cp sudachi-dictionary-20210802/system_core.dic #{dest_dir}/system.dic",
      "rm -rf sudachi-dictionary-20210802",
      "rm sudachi-dictionary-20210802-core.zip",
    ]
    |> Enum.each(&System.shell/1)
  end
end
