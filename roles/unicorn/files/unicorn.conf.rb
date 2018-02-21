worker_processes 12
user "lobsters"
working_directory "/srv/lobste.rs/http"
listen "/srv/lobste.rs/run/unicorn.sock", :backlog => 64
pid "/srv/lobste.rs/run/unicorn.pid"
timeout 30

stderr_path "/srv/lobste.rs/log/unicorn.stderr.log"
stdout_path "/srv/lobste.rs/log/unicorn.stdout.log"

preload_app true

before_fork do |server, worker|
  if defined?(ActiveRecord::Base)
    ActiveRecord::Base.connection.disconnect!
  end

  old_pid = "#{server.config[:pid]}.oldbin"
  if old_pid != server.pid
    begin
      sig = (worker.nr + 1) >= server.worker_processes ? :QUIT : :TTOU
      Process.kill(sig, File.read(old_pid).to_i)
    rescue Errno::ENOENT, Errno::ESRCH
    end
  end

  system("rm -rf /srv/lobste.rs/cache")
end

after_fork do |server, worker|
  if defined?(ActiveRecord::Base)
    ActiveRecord::Base.establish_connection
  end
end
