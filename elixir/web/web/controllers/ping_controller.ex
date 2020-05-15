defmodule Web.PingController do
  use Web.Web, :controller

  def index(conn, _params) do
    json(conn, %{value: "pong"})
  end
end
