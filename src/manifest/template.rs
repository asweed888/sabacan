use askama::Template;


#[derive(Template)]
#[template(source = "lang: {{ lang }}
{% if is_ddd -%}
arch: ddd
{% endif -%}
spec:
{% if !is_ddd -%}
- location: character
  codefile:
    - name: entity
{% else -%}
- location: domain
  upstream:
    - name: model
      upstream:
        - name: character
          codefile:
            - name: entity

    - name: repository
      codefile:
        - name: character


- location: infrastructure
  upstream:
    - name: repository
      codefile:
        - name: character


- location: usecase
  codefile:
    - name: character


- location: presentation
  upstream:
    - name: http
      upstream:
        - name: handler
          codefile:
            - name: character

- location: di
  codefile:
    - name: container
{% endif -%}
", ext = "txt")]
pub struct ManifestTemplate {
    pub lang: String,
    pub is_ddd: bool,
}