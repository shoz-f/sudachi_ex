defmodule Sudachi.NIF do
  use Rustler,
    otp_app: :sudachi,
    crate: :sudachi_nif,
    env: [{"SUDACHI_RESOURCES", Application.app_dir(:sudachi, "priv/resources")}]

  @moduledoc false

  # When loading a NIF module, dummy clauses for all NIF function are required.
  # NIF dummies usually just error out when called when the NIF is not loaded, as that should never normally happen.
  def analize(_arg1, _arg2, _arg3), do: :erlang.nif_error(:nif_not_loaded)
end
