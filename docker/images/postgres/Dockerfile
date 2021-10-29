FROM postgres:latest

ENV POSTGRES_PASSWORD=timer
ENV POSTGRES_USER=timer
ENV POSTGRES_DB=timer

ENV PGUSER=timer

COPY entrypoint.sh /run/entrypoint.sh
RUN chmod +x /run/entrypoint.sh

RUN apt upgrade && apt update \
  && apt install -y git vim make sudo

RUN PERL_MM_USE_DEFAULT=1 \
  cpan TAP::Parser::SourceHandler::pgTAP && \
  git clone https://github.com/theory/pgtap.git

CMD ["/run/entrypoint.sh"]
