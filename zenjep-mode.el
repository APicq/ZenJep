(define-minor-mode zenjep-mode
  "Toggle zenjep-mode.
A mode two add convenient functions when editing yaml flight file"
  ;; The initial value.
  :global nil
  ;; The indicator for the mode line.
  :lighter " ZenJep"
  ;; The minor mode bindings.
  :keymap '(
	    ("\C-qd" . zenjep-add-takeoff-day)
	    ("\C-qn" . zenjep-add-takeoff-night)
	    ("\C-qi" . zenjep-oc-time-ifr)
	    ("\C-qs" . zenjep-oc-time-night))

  (defun zenjep-add-takeoff-day ()
    "Add takeoff and landing day"
    (interactive)
    (end-of-line)
    (insert "\n  takeoff_day: 1\n  landing_day: 1"))

  (defun zenjep-add-takeoff-night ()
    "Add takeoff and landing night"
    (interactive)
    (end-of-line)
    (insert "\n  takeoff_night: 1\n  landing_night: 1"))

  (defun zenjep-oc-time-ifr()
    "Add operational time ifr. The cursor must be on the line
with the duration_total."
    (interactive)
    (beginning-of-line)
    (setq first-mark (point))
    (end-of-line)
    (setq second-mark (point))
    (setq duration-tot (buffer-substring-no-properties first-mark second-mark))
    (setq result
	  (replace-regexp-in-string "duration_total" "oc_time_ifr" duration-tot))
    (insert "\n")
    (insert result))

  (defun zenjep-oc-time-night()
    "Add operational time night. The cursor must be on the line
with the duration_total."
    (interactive)
    (beginning-of-line)
    (setq first-mark (point))
    (end-of-line)
    (setq second-mark (point))
    (setq duration-tot (buffer-substring-no-properties first-mark second-mark))
    (setq result
	  (replace-regexp-in-string "duration_total" "oc_time_night" duration-tot))
    (insert "\n")
    (insert result)))


;; test zone
- date_start: "2017-10-05 14:20"
date_end: "2017-10-05 16:50"
duration_total: "02:33"
oc_time_ifr: "02:33"
oc_time_ifr: "02:30"
oc_time_ifr: "02:30"
takeoff_day: 1
landing_day: 1
apt_departure_iata: PRN
apt_arrival_iata: BSL
apt_arrival_icao: LFSB
apt_arrival_name: EuroAirport Basel-Mulhouse
immatriculation: YRTIB


(defun zenjep-add-flight(date duration from to)
  "a test function"
  (interactive "ss1:\ns2:\n")
  (message "s1=%s s2=%s" s1 s2))
