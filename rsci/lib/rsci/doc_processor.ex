defmodule RSci.DocProcessor do
  @manual_html_url "https://www.gnu.org/software/gsl/doc/html/"

  defp normalize_as_unix(text) do
    String.replace(text, "\n\r", "\n")
  end

  defp math_image?(img) do
    case Floki.attribute(img, "src") do
      [src] ->
        String.starts_with?(src, "_images/math/")

      _other ->
        false
    end
  end

  defp html_to_md_helper(html) do
    Floki.traverse_and_update(html, fn
      {"div", [{"class", "math"}], [text, "\n\n"]} when is_binary(text) ->
        ["$#{text}$", "\n\n"]

      {"img", _attrs, _contents} = img ->
        if math_image?(img) do
          [inline_math] = Floki.attribute(img, "alt")
          "$#{inline_math}$"

        else
          [alt] = Floki.attribute(img, "alt")
          [src] = Floki.attribute(img, "src")

          "![#{alt}](https://www.gnu.org/software/gsl/doc/html/#{src})\n\n"
        end

      {"p", _attrs, contents} ->
        contents ++ ["\n\n"]

      {"code", _attrs, contents} ->
        ["`"] ++ contents ++ ["`"]

      {"span", _attrs, contents} ->
        contents

      {_tag, _attrs, contents} ->
        contents

      other ->
        other
    end)
  end

  # def html_to_text(html) do
  def extract_function_signature_from_html_docs(html) do
    iolist = Floki.traverse_and_update(html, fn
      {_tag, _attr, contents} ->
        contents

      other ->
        other
    end)

    iolist
    |> List.flatten()
    |> Enum.intersperse(" ")
    |> Enum.join()
    |> String.replace("¶", "")
    |> String.trim()
  end

  def html_doc_to_md(html, module_name, gsl_function_name) do
    md_list = html_to_md_helper(html)

    url = "#{@manual_html_url}/#{module_name}.html#c.#{gsl_function_name}"

    reference_line = "Binds the function [`#{gsl_function_name}`](#{url})."

    md_doc = IO.iodata_to_binary([md_list, reference_line])

    md_doc
    |> normalize_as_unix()
    |> String.replace(~r/\.\s+(?=\w)/, ".\n\n", global: false)
    |> String.split("\n\n")
    |> Enum.map(fn line -> Excribe.format(line, %{width: 80}) end)
    |> Enum.join("\n\n")
  end
end
